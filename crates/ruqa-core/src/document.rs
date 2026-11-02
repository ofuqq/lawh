//! Document model and canonical `.ruqa` versioned JSON persistence.
//!
//! Handles serialization and deserialization of Ruqa whiteboard documents,
//! schema validation, forward compatibility, and file I/O operations.

use crate::element::Element;
use crate::error::DocumentError;
use crate::geometry::Viewport;
use crate::scene::Scene;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Expected canonical file extension for Ruqa whiteboard documents.
pub const FILE_EXTENSION: &str = "ruqa";

/// Canonical format identifier string.
pub const FORMAT_MAGIC: &str = "ruqa";

/// Supported schema version.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Default canvas background color (dark theme).
pub const DEFAULT_BACKGROUND_COLOR: &str = "#121212";

/// Metadata describing the canvas environment (background, viewport).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasMeta {
    /// Canvas background color in hex format (e.g. `"#121212"`).
    #[serde(default = "default_background_color")]
    pub background_color: String,
    /// Infinite canvas viewport camera state.
    #[serde(default)]
    pub viewport: Viewport,
}

fn default_background_color() -> String {
    DEFAULT_BACKGROUND_COLOR.to_string()
}

impl Default for CanvasMeta {
    fn default() -> Self {
        Self {
            background_color: default_background_color(),
            viewport: Viewport::default(),
        }
    }
}

/// Root JSON document representation of a `.ruqa` whiteboard file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuqaDocument {
    /// Format identifier; must be `"ruqa"`.
    pub format: String,
    /// Format schema version integer (e.g. `1`).
    pub schema_version: u32,
    /// Application semantic version string that created this file.
    pub app_version: String,
    /// Optional ISO 8601 timestamp string when the document was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Optional ISO 8601 timestamp string when the document was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Canvas settings and viewport camera state.
    #[serde(default)]
    pub canvas: CanvasMeta,
    /// Ordered list of vector shapes and elements in z-index order.
    #[serde(default)]
    pub elements: Vec<Element>,
}

impl RuqaDocument {
    /// Creates a new document from a [`Scene`] and [`Viewport`].
    #[must_use]
    pub fn new(scene: &Scene, viewport: Viewport) -> Self {
        Self {
            format: FORMAT_MAGIC.to_string(),
            schema_version: CURRENT_SCHEMA_VERSION,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: None,
            updated_at: None,
            canvas: CanvasMeta {
                background_color: DEFAULT_BACKGROUND_COLOR.to_string(),
                viewport,
            },
            elements: scene.elements().to_vec(),
        }
    }

    /// Converts this document into its constituent [`Scene`] and [`Viewport`].
    #[must_use]
    pub fn into_scene_and_viewport(self) -> (Scene, Viewport) {
        let scene = Scene::from_elements(self.elements);
        let viewport = self.canvas.viewport;
        (scene, viewport)
    }

    /// Serializes this document into pretty-printed UTF-8 JSON.
    pub fn to_json_string(&self) -> Result<String, DocumentError> {
        serde_json::to_string_pretty(self).map_err(|e| DocumentError::Parse {
            path: PathBuf::from("<memory>"),
            source: Box::new(e),
        })
    }

    /// Deserializes and validates a document from a JSON string.
    pub fn from_json_str(json: &str) -> Result<Self, DocumentError> {
        Self::from_json_str_with_path(json, Path::new("<memory>"))
    }

    /// Deserializes, validates, and automatically migrates a document from a JSON string with an associated path for error reporting.
    pub fn from_json_str_with_path(json: &str, path: &Path) -> Result<Self, DocumentError> {
        let mut value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| DocumentError::Parse {
                path: path.to_path_buf(),
                source: Box::new(e),
            })?;

        let format = value
            .get("format")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        if format != FORMAT_MAGIC {
            return Err(DocumentError::Parse {
                path: path.to_path_buf(),
                source: format!(
                    "invalid format identifier '{}'; expected '{}'",
                    format, FORMAT_MAGIC
                )
                .into(),
            });
        }

        let schema_version = value
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(0);

        if schema_version > CURRENT_SCHEMA_VERSION {
            return Err(DocumentError::UnsupportedVersion {
                path: path.to_path_buf(),
                found: schema_version.to_string(),
                max_supported: CURRENT_SCHEMA_VERSION.to_string(),
            });
        }

        if schema_version < CURRENT_SCHEMA_VERSION {
            crate::migration::default_pipeline().migrate(
                &mut value,
                CURRENT_SCHEMA_VERSION,
                path,
            )?;
        }

        let doc: Self = serde_json::from_value(value).map_err(|e| DocumentError::Parse {
            path: path.to_path_buf(),
            source: Box::new(e),
        })?;

        Ok(doc)
    }

    /// Loads and parses a `.ruqa` file from disk.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, DocumentError> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path).map_err(|e| DocumentError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        Self::from_json_str_with_path(&contents, path)
    }

    /// Returns the sibling temporary file path used during atomic saving.
    ///
    /// For a target path like `diagram.ruqa` or `/path/to/diagram.ruqa`, this returns
    /// `.diagram.ruqa.tmp` or `/path/to/.diagram.ruqa.tmp` in the same directory.
    #[must_use]
    pub fn temp_file_path(path: &Path) -> PathBuf {
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("document.ruqa");
        parent.join(format!(".{file_name}.tmp"))
    }

    /// Saves this document to disk as formatted UTF-8 JSON atomically with crash safety.
    ///
    /// The document payload is first written to a temporary sibling file (`.<filename>.tmp`)
    /// in the same directory, flushed and synced to storage media, and then atomically renamed
    /// over the target file path (`std::fs::rename`). This guarantees the file on disk is never
    /// left partially written or corrupted in the event of an unexpected crash or power loss.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), DocumentError> {
        let path = path.as_ref();
        let json = self.to_json_string().map_err(|mut e| {
            if let DocumentError::Parse {
                path: ref mut p, ..
            } = e
            {
                *p = path.to_path_buf();
            }
            e
        })?;

        let temp_path = Self::temp_file_path(path);
        let write_result = (|| -> Result<(), std::io::Error> {
            use std::io::Write;
            let mut file = std::fs::File::create(&temp_path)?;
            file.write_all(json.as_bytes())?;
            file.sync_all()?;
            drop(file);
            std::fs::rename(&temp_path, path)?;
            Ok(())
        })();

        if let Err(e) = write_result {
            let _ = std::fs::remove_file(&temp_path);
            return Err(DocumentError::Io {
                path: path.to_path_buf(),
                source: e,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{ElementStyle, FillStyle};
    use crate::geometry::{Point, Rect};

    #[test]
    fn test_document_round_trip_json() {
        let mut scene = Scene::new();
        scene.push(Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(10.0, 20.0, 100.0, 50.0),
            style: ElementStyle {
                stroke_color: "#E0E0E0".into(),
                stroke_width: 2.0,
                fill_color: Some("#333333".into()),
                fill_style: FillStyle::Solid,
                roughness: 1.2,
                seed: 42,
            },
        });

        let viewport = Viewport::new(Point::new(100.0, -50.0), 1.5);
        let doc = RuqaDocument::new(&scene, viewport);

        let json = doc.to_json_string().unwrap();
        assert!(json.contains("\"format\": \"ruqa\""));
        assert!(json.contains("\"schema_version\": 1"));

        let loaded = RuqaDocument::from_json_str(&json).unwrap();
        assert_eq!(doc, loaded);

        let (restored_scene, restored_viewport) = loaded.into_scene_and_viewport();
        assert_eq!(restored_scene.len(), 1);
        assert_eq!(restored_viewport.pan, Point::new(100.0, -50.0));
        assert_eq!(restored_viewport.zoom, 1.5);
    }

    #[test]
    fn test_forward_compatibility_ignores_unknown_fields() {
        let json = r##"{
            "format": "ruqa",
            "schema_version": 1,
            "app_version": "0.1.0",
            "future_metadata": {
                "author": "Alice",
                "custom_tags": [1, 2, 3]
            },
            "canvas": {
                "background_color": "#121212",
                "extra_grid_setting": true,
                "viewport": {
                    "pan": { "x": 10.0, "y": 20.0 },
                    "zoom": 1.0,
                    "future_zoom_mode": "logarithmic"
                }
            },
            "elements": []
        }"##;

        let doc = RuqaDocument::from_json_str(json).unwrap();
        assert_eq!(doc.format, "ruqa");
        assert_eq!(doc.canvas.viewport.pan, Point::new(10.0, 20.0));
    }

    #[test]
    fn test_rejects_invalid_format() {
        let json = r#"{
            "format": "foreign_format",
            "schema_version": 1,
            "app_version": "0.1.0",
            "elements": []
        }"#;

        let err = RuqaDocument::from_json_str(json).unwrap_err();
        assert!(err.to_string().contains("invalid format identifier"));
    }

    #[test]
    fn test_rejects_future_schema_version() {
        let json = r#"{
            "format": "ruqa",
            "schema_version": 99,
            "app_version": "2.0.0",
            "elements": []
        }"#;

        let err = RuqaDocument::from_json_str(json).unwrap_err();
        match err {
            DocumentError::UnsupportedVersion { found, .. } => {
                assert_eq!(found, "99");
            }
            other => panic!("expected UnsupportedVersion, got {other:?}"),
        }
    }
}
