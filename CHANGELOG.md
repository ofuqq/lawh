# Changelog

All notable changes to Ruqa will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Contextual styling HUD panel in `ruqa-render` and `ruqa-app` featuring 6 curated dark-theme color swatches (White, Coral, Yellow, Green, Cyan, Lavender), 3 stroke width presets (Thin 1.0, Medium 2.5, Bold 4.0), and roughness toggle (Clean 0.0 vs Sketchy 1.2) (fixes #28).
- Selection-aware style mutations applying property changes to selected elements as undoable `Command::ModifyElements` history actions and updating default creation styles when nothing is selected (fixes #28).
- Selection style synchronization (`sync_style_from_selection`) updating HUD panel indicators when shapes are selected (fixes #28).
- Single-key keyboard shortcuts (`V`, `P`, `R`, `O`/`C`, `D`, `L`, `A`, `E`, `T`) in `ruqa-app` for instant drawing tool switching (fixes #27).
- Text editing suppression logic (`is_editing_text`) preventing accidental tool switches while typing (fixes #27).
- `Tool::from_shortcut` helper with alias support ('O' and 'C' for Ellipse) and `Tool::Text` variant in `ruqa-core` (fixes #27).
- Minimal floating top HUD toolbar for tool selection in `ruqa-render` and `ruqa-app` featuring rounded dark theme styling, active badge highlights, and custom vector icons for all 8 tools (Select, Pen, Rectangle, Ellipse, Diamond, Line, Arrow, Eraser) (fixes #26).
- Canvas click isolation and hit-testing preventing toolbar clicks from placing shapes or selecting elements underneath (fixes #26).
- `Tool` enumeration and canonical tool definitions in `ruqa-core` (fixes #26).
- Atomic crash-safe document saving in `ruqa-core` writing payloads to sibling temporary files (`.<filename>.tmp`) with disk syncing (`sync_all`) before atomic replacement (`std::fs::rename`) (fixes #54).
- Automatic background autosave in `ruqa-app` triggered after 30 seconds of inactivity when documents are dirty and have assigned file paths (fixes #54).
- Document dirty state tracking (`is_dirty: bool`) and dynamic OS window title updates (`Ruqa - filename.ruqa*`, `Ruqa - Untitled*`) in `ruqa-app` (fixes #25).
- File operations module (`file_actions`) with `Ctrl+S` document saving, `save_as`, `open_file`, and `new_document` session workflows (fixes #25).
- Document schema migration framework (`Migration`, `MigrationPipeline`, `V0ToV1Migration`) in `ruqa-core` executing sequential AST transforms over raw JSON before domain struct deserialization (fixes #53).
- Canonical document persistence (`RuqaDocument`, `CanvasMeta`) in `ruqa-core` serializing viewport camera and all scene elements in versioned `.ruqa` JSON format with schema version validation and forward compatibility (fixes #24).
- File I/O helpers (`save_to_file`, `load_from_file`, `to_json_string`, `from_json_str`) with structured `DocumentError` diagnostics (fixes #24).
- Duplicate and copy/paste selected elements with `Ctrl+D`, `Ctrl+C`, and `Ctrl+V` in `ruqa-app` with cursor-centered pasting, fresh ID generation, roughness PRNG seed re-randomization, and full undo/redo integration (fixes #52).
- Element duplication helper methods (`Element::duplicate_with`, `Element::set_id`) in `ruqa-core` (fixes #52).
- Deleting single and multi-selected canvas elements with `Delete` and `Backspace` keys in `ruqa-app` (fixes #51).
- Atomic `DeleteElements` command registration in `HistoryStack` enabling instant `Ctrl+Z` restoration of deleted shapes with original IDs and exact z-order layer positions (fixes #51).
- Unlimited undo and redo history using the command pattern (`HistoryStack`, `Command`) in `ruqa-core` supporting discrete reversible actions (add, delete, move, modify), configurable depth, and linear history truncation (fixes #23).
- Wired standard keyboard shortcuts in `ruqa-app`: Ctrl+Z for undo, Ctrl+Y and Ctrl+Shift+Z for redo, with macOS Command (Super) key mapping (fixes #23).
- Automatic generation and emission of reversible `Command::MoveElements` and `Command::ModifyElements` upon translation and resize gestures in `ruqa-app` (fixes #23).
- Interactive scaling and resizing of selected canvas elements from 8 selection handles in `ruqa-app` with opposite-corner anchor pinning, single-axis edge dragging, Shift aspect ratio constraint, and 5x5 px minimum dimension clamping (fixes #22).
- Resize geometry math (`compute_resized_bounds`, `HandleKind`, `MIN_RESIZE_DIMENSION`) and proportional element scaling (`Element::scale_from_bounds`) in `ruqa-core` (fixes #22).
- Extracted `SelectionState` and `DragMode` submodules in `ruqa-app` and `style` module in `ruqa-core` keeping all source modules under ~350 LOC (fixes #22).
- Interactive drag translation of single and multi-selected canvas elements in `ruqa-app` tracking cursor motion smoothly across viewport zoom levels (fixes #21).
- Translation methods on geometric primitives and scene graph (`Element::translate`, `Rect::translate`, `Scene::translate_elements`) in `ruqa-core` (fixes #21).
- Selection outline and 8 resize handles rasterizer (`render_selection`) in `ruqa-render` with constant 8x8 screen-pixel handle size regardless of camera zoom (fixes #20).
- Multi-selection aggregate bounding box calculation (`Scene::selection_bounds`) in `ruqa-core` enclosing single and multiple selected elements (fixes #20).
- Modularized OS window event handling in `ruqa-app` via dedicated `window_handler` keeping all source modules under ~350 LOC (fixes #20).
- Selection tool interaction in `ruqa-app` supporting single-click select, Shift-click toggle, empty canvas deselect, and drag-marquee intersection selection (fixes #19).
- Semi-transparent selection marquee overlay rasterization (`render_marquee`) in `ruqa-render` (fixes #19).
- Precise point and boundary distance hit-testing (`Element::hit_test`, `Scene::hit_test_topmost`) in `ruqa-core` distinguishing filled interiors from stroke perimeters (fixes #18).
- Tight axis-aligned bounding box calculation (`Element::bounding_box`) in `ruqa-core` with stroke width padding, arrowhead barb geometry, and rectangle inflation (fixes #17).
- Freehand variable-width ink polygon rasterizer in `ruqa-render` with rounded end caps and zero-allocation scratch buffers for 60 FPS drawing (fixes #16).
- Freehand pen stroke smoothing in `ruqa-core` via Catmull-Rom spline interpolation and pointer velocity pressure modulation (fixes #15).
- Hand-drawn sketchy geometric shape renderer (`Rectangle`, `Ellipse`, `Diamond`, `Line`, `Arrow`) in `ruqa-render` with stable seed reproducibility and viewport culling (fixes #14).
- Core `Scene` model in `ruqa-core` with monotonic element IDs, z-index reordering, spatial aggregate bounds, and viewport visibility culling (fixes #50).
- Geometric shape primitives (`Rectangle`, `Ellipse`, `Diamond`, `Line`, `Arrow`), `ElementStyle`, and `FillStyle` in `ruqa-core` with Serde JSON round-trip serialization (fixes #13).
- Converter from `roughr` operation sets and drawables to `tiny_skia::Path` with open/closed path conversion and round stroke capping (fixes #12).
- Pan and zoom canvas interaction in `ruqa-app` via middle-mouse drag, Space + left drag, and cursor-anchored wheel zooming with phantom-click suppression (fixes #11).
- 2D canvas viewport camera and coordinate transforms in `ruqa-core` with anchored zooming and visible rect calculation (fixes #10).
- Desktop window initialization, softbuffer frame presentation, and HiDPI coordinate scaling with headless smoke-test harness (fixes #8, #9, #49, #47).
- Initial project bootstrap, final naming as Ruqa (from Ruq'ah, the Arabic handwriting style), and governance framework.
- Apache-2.0 licensing with NOTICE and `cargo-deny` policy enforcement.
- Product documentation: vision, principles, users and use cases, competitive analysis, MVP scope, and kill criteria gates.
- Technical architecture specifications: 3-crate layout (`ruqa-core`, `ruqa-render`, `ruqa-app`), data model, file format (`.ruqa`), CPU rendering pipeline, text/RTL shaping via Parley, and testing strategy.
- Architectural Decision Records: ADR 0001 through ADR 0009.
- Market and technical research synthesis digest (`docs/research/`).
- GitHub Actions CI with pinned SHAs: format, clippy, unit tests, and `cargo-deny` license/vulnerability audit.
- GitHub issue templates (bug, feature, task, spike), PR template with AI disclosure, labels taxonomy, and main branch ruleset.
- Minimal Rust workspace skeleton with `ruqa-core` crate and baseline unit tests.
