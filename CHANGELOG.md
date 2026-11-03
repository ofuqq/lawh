# Changelog

All notable changes to Ruqa will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Integrate Parley text layout and Skrifa outline rasterization in `ruqa-render` and `ruqa-core` (`Element::Text`, `render_text_element`) for bilingual text rendering on canvas (fixes #39).
- Font documentation and licensing guide in `assets/fonts/README.md` for bundled Aref Ruqaa (OFL-1.1) and Roboto (Apache-2.0) fonts (fixes #40).
- Sample demo board CLI flag (`--demo`) in `ruqa-app` (`populate_demo_board`, `load_demo_board`) seeding the canvas with hand-drawn rectangle, ellipse, diamond, arrow, line, and freehand pen stroke for visual exploration and interaction testing (fixes #84).

### Fixed
- Apply HiDPI scale factor to pixmap rendering transforms in `ruqa-render` and `ruqa-app` (`render_scene_viewport_scaled`, `render_selection_scaled`, `render_grid_scaled`, `render_recent_files_hud_scaled`, `render_vault_sidebar_scaled`, `render_quit_dialog_scaled`) ensuring full-bleed, sharp rendering across Retina and scaled displays (fixes #90).

## [0.1.0] - 2026-11-03

### Highlights
- Initial public MVP release of Ruqa: a lightweight, mouse-first open-source desktop whiteboard built in native Rust.
- Hand-drawn vector aesthetics powered by `roughr` and pure CPU rendering via `tiny-skia` presented through `softbuffer`.
- Full direct manipulation (selection, translation, 8-handle proportional resizing, continuous eraser, undo/redo).
- File persistence in versioned `.ruqa` JSON format with atomic disk writing and automatic autosave.
- Built-in multi-board organization with the local Study Vault sidebar (`Ctrl+B`) and Recent Files Welcome HUD (`Ctrl+O`).
- Production-grade high-resolution PNG export (`Ctrl+E`) supporting tight selection bounds vs full board framing, transparent or dark backgrounds, and 1x/2x scale.
- Validated performance: 60 FPS viewport rendering up to 10,000 shapes, sustained 34.2 MB peak memory footprint during continuous 2-hour dogfooding session with zero memory leaks.

### Supported Platforms & System Requirements
- **Linux**: x86_64 and aarch64 under X11 or Wayland (Ubuntu 22.04+ or equivalent glibc 2.35+ distro).
  - Runtime & build dependencies: `libxkbcommon-dev`, `libxkbcommon-x11-dev`, `libgl1-mesa-dev`.
- **macOS**: x86_64 (Intel macOS 12+) and aarch64 (Apple Silicon macOS 14+). Native Cocoa windowing without external display server dependencies.

### Verified Keyboard & Mouse Shortcuts
| Action / Tool | Shortcut | Description |
|---|---|---|
| **Select Tool** | `V` | Single click select, marquee drag multi-select |
| **Pen Tool** | `P` | Freehand smooth variable-width ink |
| **Rectangle Tool** | `R` | Hand-drawn sketchy rectangle |
| **Ellipse Tool** | `O` or `C` | Hand-drawn sketchy ellipse |
| **Diamond Tool** | `D` | Hand-drawn sketchy decision diamond |
| **Line Tool** | `L` | Straight vector line |
| **Arrow Tool** | `A` | Directed arrow connector with arrowhead barbs |
| **Eraser Tool** | `E` | Continuous drag eraser removing contacted elements |
| **Pan Canvas** | `Middle Mouse Drag` or `Space + Left Drag` | Infinite 2D pan |
| **Zoom Canvas** | `Mouse Wheel` | Smooth cursor-anchored 2D zoom (0.1x to 10.0x) |
| **Undo** | `Ctrl + Z` / `Cmd + Z` | Revert previous canvas action |
| **Redo** | `Ctrl + Y` / `Ctrl + Shift + Z` | Reapply undone canvas action |
| **Duplicate** | `Ctrl + D` | Clone selected elements offset by (+20, +20) |
| **Copy / Paste** | `Ctrl + C` / `Ctrl + V` | Copy to clipboard and paste at cursor position |
| **Delete** | `Delete` / `Backspace` | Delete active selection |
| **Save Document** | `Ctrl + S` | Atomic crash-safe document save |
| **Export PNG** | `Ctrl + E` | Export active selection or whole diagram to PNG |
| **Study Vault** | `Ctrl + B` | Toggle collapsible local study vault sidebar |
| **Recent Files HUD** | `Ctrl + O` | Toggle recent boards welcome overlay |
| **Orientation Grid** | `Ctrl + '` | Toggle canvas dot/graph grid |
| **Invert Theme** | `Ctrl + I` | Toggle between Dark and Light canvas themes |
| **Quit** | `Ctrl + Q` / `Ctrl + W` | Intercept window close with prompt if unsaved |

### File Format Specification (`.ruqa`)
Ruqa documents persist as UTF-8 JSON files bearing the `.ruqa` extension with schema versioning:
- **Format Identifier**: `"format": "ruqa"`
- **Schema Version**: `"schema_version": 1`
- **Canvas Metadata**: Viewport camera position (`camera_x`, `camera_y`), zoom factor (`zoom`), and background color.
- **Scene Elements**: Ordered array of serialized elements (`type`, `id`, `z_index`, `roughness_seed`, `style`, geometry coordinates).
- **Forward Compatibility**: Built-in document migration pipeline (`MigrationPipeline`) supporting seamless forward AST upgrades without data loss.

### Added
- Unsaved changes quit confirmation dialog in `ruqa-render` and `ruqa-app` (`QuitDialogLayout`, `QuitDialogAction`, `render_quit_dialog`, `request_close`, `handle_quit_dialog_pointer_down`, `handle_quit_dialog_key`) intercepting window close requests and `Ctrl+Q`/`Ctrl+W` shortcuts to prompt with "Save", "Don't Save", or "Cancel" when modified, while cleanly exiting immediately when clean (fixes #35).
- Modularized pointer input event dispatch into `pointer_input` in `ruqa-app` keeping application harness modules strictly under the module size limits (fixes #35).
- Selection vs full diagram bounds export support in `ruqa-render` and `ruqa-app` (`export_elements_to_pixmap`, `export_elements_to_png`, `export_elements_to_png_file`, `has_active_selection`, `export_elements`) isolating selected elements with tight bounding boxes plus padding, or enclosing all scene elements when no selection is active, avoiding empty canvas margins (fixes #34).
- Headless high-resolution PNG image export pipeline in `ruqa-render` (`export_png`, `PngExportOptions`, `export_scene_to_pixmap`, `export_scene_to_png`, `export_scene_to_png_file`) supporting custom resolution scale multipliers (1x, 2x HiDPI), transparent vs dark backgrounds, outer padding margins, and standard PNG encoding (fixes #33).
- PNG export operations and shortcuts in `ruqa-app` (`export_actions`, `Ctrl+E`) computing aggregate board bounds and exporting crisp diagrams directly to disk (fixes #33).
- `RenderError::PngEncode` error variant in `ruqa-core` capturing PNG encoding diagnostics (fixes #33).
- Local Study Vault hierarchy scanner and data model in `ruqa-core` (`StudyVault`, `FlatVaultItem`, `MAX_VAULT_DEPTH: 6`, `RUQA_EXTENSION: "ruqa"`) scanning folders and boards with expand/collapse tracking (fixes #56).
- Study Vault drawer sidebar layout and rasterizer in `ruqa-render` (`VaultLayout`, `render_vault_sidebar`, `VaultTheme`) featuring left panel drawer, header actions, chevron toggles, folder icons, board icons, active board indicators, and hover highlights (fixes #56).
- Study Vault sidebar integration in `ruqa-app` (`vault_actions`) toggling via `Ctrl+B`, expanding/collapsing subfolders, autosaving dirty documents when opening boards, and isolating clicks from canvas (fixes #56).
- Recent files list tracking and LRU management in `ruqa-core` (`RecentFiles`, `RecentFileEntry`, `MAX_RECENT_FILES: 10`) persisting to `~/.config/ruqa/recent_files.json` across platforms without external dependencies (fixes #55).
- Recent Files / Welcome HUD modal overlay in `ruqa-render` (`render_recent_files_hud`, `RecentFilesLayout`, `RecentFilesTheme`) displaying recent boards, document icons, last modified timestamps, and quick "New Board" / dismiss buttons (fixes #55).
- Interactive Welcome HUD interaction in `ruqa-app` (`recent_files_actions`) loading documents directly on click, isolating clicks from background canvas, updating hover states, toggling via `Ctrl+O`, and dismissing on `Escape` or backdrop click (fixes #55).
- CLI initial file argument handling in `ruqa-app` opening specified `.ruqa` boards directly and bypassing the startup HUD (fixes #55).
- Spatial axis-aligned bounding box (AABB) viewport culling in `ruqa-core` (`Scene::visible_elements`) with stroke width padding, arrowhead barb geometry, and rectangle bounds inflation (fixes #32).
- Viewport culling rasterizer pass in `ruqa-render` (`render_scene_viewport`) skipping non-intersecting offscreen canvas shapes and returning rendered element counts (fixes #32).
- Performance verification and benchmark suite in `ruqa-render` (`culling_tests`) proving rendering latency scales with on-screen visible elements rather than total board elements, sustaining $\ge 60\text{ FPS}$ on multi-thousand element boards (fixes #32).
- Default dark theme palette and color constants in `ruqa-core` (`DARK_BACKGROUND_COLOR: "#121212"`, `DARK_PEN_INK: "#ECEFF4"`, `LIGHT_BACKGROUND_COLOR: "#FFFFFF"`, `LIGHT_PEN_INK: "#1E1E1E"`, `ThemeMode`) (fixes #31).
- Perceptual color luminance, HSV saturation, and selective color inversion helpers (`invert_color`, `invert_element_style`, `invert_element`, `invert_scene`) inverting monochrome ink while strictly preserving chromatic accent colors (fixes #31).
- Zero-allocation canvas orientation grid renderer in `ruqa-render` (`render_grid`, `GridConfig`, `GridStyle`) supporting adaptive zoom-scaled dot and graph line rendering directly into pixmaps (fixes #31).
- Canvas background grid toggling via `Ctrl+'` keyboard shortcut in `ruqa-app` and seamless `toggle_theme` workspace support (fixes #31).
- Continuous stroke and element eraser tool in `ruqa-app` (`EraserState`, `handle_eraser_pointer_down`, `handle_eraser_pointer_move`, `handle_eraser_pointer_up`) removing touched shapes and freehand ink paths immediately upon contact (fixes #30).
- Drag trail segment interpolation sampling at 75% tolerance intervals to prevent missed strokes during fast cursor swipes (fixes #30).
- Atomic gesture deletion committing all erased elements as a single `Command::DeleteElements` on pointer release, restoring all erased shapes with preserved z-order on Ctrl+Z (fixes #30).
- Modularized frame rendering pass (`frame_render`) keeping `window_app` well under the ~350 LOC limit (fixes #30).
- Organic fill pattern rendering in `ruqa-render` supporting `Solid`, `SemiTransparent` (50% opacity), `Hachure` (hand-drawn parallel angled sketch lines), and `CrossHatch` (intersecting hatching) for closed shapes (rectangles, ellipses, diamonds) (fixes #29).
- Hand-drawn fill jitter generation matching stroke roughness and deterministic seed stability across rendering frames (fixes #29).
- Separated fill path rasterization (`OpSetType::FillPath`) and sketch hatching rasterization (`OpSetType::FillSketch`) from outline strokes (`OpSetType::Path`) in `shape_render` and `rough_options` (fixes #29).
- `FillStyle::SemiTransparent` variant and helper methods (`is_filled`, `ALL`) in `ruqa-core` (fixes #29).
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
