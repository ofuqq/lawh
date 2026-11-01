# File Format Specification (`.ruqa`)

> **Status: Design Proposal (Draft)**  
> This document defines the canonical on-disk storage format and Study Vault structure for Ruqa.

---

## 1. Design Principles

1. **Human-Readable & Git-Diffable**: Stored as clean, UTF-8 encoded JSON.
2. **Explicit Schema Versioning**: Top-level `schema_version: 1` and `format: "ruqa"` magic keys.
3. **Atomic File Writes**: Document saves write first to a temporary file (`.board.ruqa.tmp`) and atomically rename it into place, preventing corruption during system crashes.
4. **Vault-as-a-Folder**: A Study Vault is simply a standard operating system folder containing `.ruqa` files and subdirectories. Zero proprietary database containers.
5. **JSON Canvas (.canvas) as Export Target**: While JSON Canvas (jsoncanvas.org) is an excellent open standard, it cannot natively represent freehand strokes or rough style parameters. Ruqa provides 1-click export to `.canvas` in v0.2 for Obsidian interoperability.

---

## 2. Schema Specification (Version 1)

File extension: `.ruqa` (or `.ruqa.json`)  
MIME type: `application/vnd.ruqa+json`

### Example Document Payload
```json
{
  "format": "ruqa",
  "schema_version": 1,
  "app_version": "0.1.0",
  "created_at": "2026-11-01T14:00:00Z",
  "updated_at": "2026-11-01T14:30:00Z",
  "canvas": {
    "background_color": "#121212",
    "viewport": {
      "pan": { "x": 0.0, "y": 0.0 },
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
      "type": "text",
      "position": { "x": 120.0, "y": 140.0 },
      "content": "خوارزمية Dijkstra تعمل في O(V log V)",
      "font_size": 18.0,
      "style": {
        "stroke_color": "#4DD0E1",
        "fill_color": null,
        "stroke_width": 1.0,
        "roughness": 0.0,
        "seed": 918237
      }
    }
  ]
}
```

---

## 3. Forward Compatibility & Migration Rules

1. **Permissive Deserialization**: The parser ignores unrecognized JSON keys without failing. A file generated in v0.3 can be read safely in v0.1.
2. **Format Verification**: The parser verifies `"format": "ruqa"` before reading to avoid attempting to parse foreign JSON files.
3. **Sequential Migration Pipeline**: When breaking changes occur in future versions, explicit migration functions ($v1 \to v2 \to v3$) update files automatically upon save.
