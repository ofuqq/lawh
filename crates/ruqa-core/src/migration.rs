//! Document schema migration pipeline for `.ruqa` version transitions.
//!
//! Provides step-wise AST migration functions executing sequentially ($v0 \to v1 \to v2$)
//! over raw JSON trees before deserialization into concrete domain structs.

use crate::error::DocumentError;
use serde_json::Value;
use std::path::Path;

/// Trait defining a single version upgrade step for a document's JSON AST.
pub trait Migration: Send + Sync {
    /// Schema version this migration starts from.
    fn source_version(&self) -> u32;

    /// Schema version this migration produces.
    fn target_version(&self) -> u32;

    /// Applies schema transformation mutations to the document JSON tree.
    fn migrate(&self, value: &mut Value) -> Result<(), DocumentError>;
}

/// Migration from legacy schema version 0 (pre-v1 drafts) to schema version 1.
#[derive(Debug, Clone, Copy, Default)]
pub struct V0ToV1Migration;

impl Migration for V0ToV1Migration {
    fn source_version(&self) -> u32 {
        0
    }

    fn target_version(&self) -> u32 {
        1
    }

    fn migrate(&self, value: &mut Value) -> Result<(), DocumentError> {
        let Some(obj) = value.as_object_mut() else {
            return Ok(());
        };

        // Ensure canvas wrapper exists
        if !obj.contains_key("canvas") {
            let mut canvas_obj = serde_json::Map::new();
            canvas_obj.insert(
                "background_color".into(),
                obj.remove("background_color")
                    .unwrap_or_else(|| Value::String("#121212".into())),
            );

            if let Some(viewport) = obj.remove("viewport") {
                canvas_obj.insert("viewport".into(), viewport);
            } else {
                let mut vp = serde_json::Map::new();
                vp.insert(
                    "pan".into(),
                    serde_json::json!({
                        "x": obj.remove("pan_x").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "y": obj.remove("pan_y").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    }),
                );
                vp.insert(
                    "zoom".into(),
                    obj.remove("zoom").unwrap_or_else(|| serde_json::json!(1.0)),
                );
                canvas_obj.insert("viewport".into(), Value::Object(vp));
            }

            obj.insert("canvas".into(), Value::Object(canvas_obj));
        }

        // Ensure format and schema_version keys are updated
        obj.insert("format".into(), Value::String("ruqa".into()));
        obj.insert("schema_version".into(), serde_json::json!(1));

        Ok(())
    }
}

/// Pipeline managing registered version migrations and sequential execution.
#[derive(Default)]
pub struct MigrationPipeline {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationPipeline {
    /// Creates a new empty migration pipeline.
    #[must_use]
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Registers a migration step in the pipeline.
    pub fn register(&mut self, migration: impl Migration + 'static) {
        self.migrations.push(Box::new(migration));
    }

    /// Executes sequential migrations until `target_version` is reached.
    pub fn migrate(
        &self,
        value: &mut Value,
        target_version: u32,
        path: &Path,
    ) -> Result<u32, DocumentError> {
        let mut current_version = value
            .get("schema_version")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(0);

        while current_version < target_version {
            let Some(migration) = self
                .migrations
                .iter()
                .find(|m| m.source_version() == current_version)
            else {
                return Err(DocumentError::UnsupportedVersion {
                    path: path.to_path_buf(),
                    found: current_version.to_string(),
                    max_supported: target_version.to_string(),
                });
            };

            let next_version = migration.target_version();
            migration.migrate(value)?;

            if let Some(obj) = value.as_object_mut() {
                obj.insert("schema_version".into(), serde_json::json!(next_version));
            }

            current_version = next_version;
        }

        Ok(current_version)
    }
}

/// Returns the default global migration pipeline supporting canonical version upgrades.
#[must_use]
pub fn default_pipeline() -> MigrationPipeline {
    let mut pipeline = MigrationPipeline::new();
    pipeline.register(V0ToV1Migration);
    pipeline
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SyntheticV1ToV2;
    impl Migration for SyntheticV1ToV2 {
        fn source_version(&self) -> u32 {
            1
        }
        fn target_version(&self) -> u32 {
            2
        }
        fn migrate(&self, value: &mut Value) -> Result<(), DocumentError> {
            if let Some(obj) = value.as_object_mut() {
                obj.insert("v2_migrated".into(), serde_json::json!(true));
            }
            Ok(())
        }
    }

    #[test]
    fn test_v0_to_v1_migration_populates_canvas_wrapper() {
        let mut json = serde_json::json!({
            "format": "ruqa",
            "schema_version": 0,
            "pan_x": 50.0,
            "pan_y": -20.0,
            "zoom": 1.5,
            "elements": []
        });

        let pipeline = default_pipeline();
        let res = pipeline.migrate(&mut json, 1, Path::new("<memory>"));
        assert_eq!(res.unwrap(), 1);

        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["canvas"]["viewport"]["pan"]["x"], 50.0);
        assert_eq!(json["canvas"]["viewport"]["pan"]["y"], -20.0);
        assert_eq!(json["canvas"]["viewport"]["zoom"], 1.5);
    }

    #[test]
    fn test_chained_multi_version_migration() {
        let mut json = serde_json::json!({
            "format": "ruqa",
            "schema_version": 0,
            "elements": []
        });

        let mut pipeline = MigrationPipeline::new();
        pipeline.register(V0ToV1Migration);
        pipeline.register(SyntheticV1ToV2);

        let res = pipeline.migrate(&mut json, 2, Path::new("<memory>"));
        assert_eq!(res.unwrap(), 2);
        assert_eq!(json["schema_version"], 2);
        assert_eq!(json["v2_migrated"], true);
    }

    #[test]
    fn test_unsupported_version_step_reports_error() {
        let mut json = serde_json::json!({
            "format": "ruqa",
            "schema_version": 0,
            "elements": []
        });

        // Pipeline with no v0 handler
        let pipeline = MigrationPipeline::new();
        let err = pipeline
            .migrate(&mut json, 1, Path::new("broken.ruqa"))
            .unwrap_err();

        match err {
            DocumentError::UnsupportedVersion { found, path, .. } => {
                assert_eq!(found, "0");
                assert_eq!(path, Path::new("broken.ruqa"));
            }
            other => panic!("expected UnsupportedVersion, got {other:?}"),
        }
    }
}
