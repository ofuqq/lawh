# File Format Specification (`.lawh`)

> **Status: Design Proposal (Draft)**  
> This document defines the canonical on-disk storage format for Lawh canvas documents.

---

## 1. Format Design Goals

1. **Human-Readable & Diffable**: Stored as clean, UTF-8 encoded JSON so users can inspect documents in text editors and track changes in Git.
2. **Explicit Versioning**: A root `version` integer enables future migrations without guessing schema shapes.
3. **Forward & Backward Compatibility**: Clients must ignore unknown properties rather than crashing when opening files created by newer versions.
4. **Self-Contained**: Contains all geometry, styles, and viewport state necessary to reconstruct the canvas identically.

---

## 2. Schema Specification (Version 1)

File extension: `.lawh`  
MIME type: `application/vnd.lawh+json`

### Example Document Payload
```json
{
  "version": 1,
  "app": "lawh",
  "created_at": "2026-11-01T14:00:00Z",
  "updated_at": "2026-11-01T14:30:00Z",
  "canvas": {
    "background_color": "#121212",
    "grid_enabled": true,
    "viewport": {
      "pan": { "x": 120.5, "y": -45.0 },
      "zoom": 1.0
    }
  },
  "elements": [
    {
      "id": 1001,
      "type": "rectangle",
      "bounds": {
        "min": { "x": 100.0, "y": 100.0 },
        "max": { "x": 350.0, "y": 250.0 }
      },
      "style": {
        "stroke_color": "#E0E0E0",
        "fill_color": null,
        "stroke_width": 2.0,
        "roughness": 1.2,
        "seed": 4829104
      }
    },
    {
      "id": 1002,
      "type": "stroke",
      "points": [
        { "x": 120.0, "y": 140.0 },
        { "x": 135.0, "y": 142.5 },
        { "x": 160.0, "y": 150.0 }
      ],
      "pressures": [0.5, 0.6, 0.7],
      "style": {
        "stroke_color": "#4DD0E1",
        "fill_color": null,
        "stroke_width": 3.0,
        "roughness": 1.0,
        "seed": 918237
      }
    }
  ]
}
```

---

## 3. Forward Compatibility & Migration Rules

1. **Ignore Unknown Fields**:
   - In Rust, deserialization structs must NOT fail on unknown attributes (default Serde behavior).
   - This ensures a file saved in Lawh v0.3 can be safely opened in Lawh v0.1 without throwing a parse error.
2. **Version Checks**:
   - If `version <= CURRENT_SUPPORTED_VERSION`, the client parses directly or applies sequential migrations ($1 \to 2 \to 3$).
   - If `version > CURRENT_SUPPORTED_VERSION`, the application warns the user that the document was created with a newer version, but attempts best-effort rendering of recognizable elements.
3. **Corrupted Element Recovery**:
   - If an individual element fails schema validation, the parser logs a warning, skips that single element, and loads the remaining valid elements.
