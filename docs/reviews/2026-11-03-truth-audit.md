# Truth Audit of "Done" (v0.1.0 Milestone Verification)

**Audit Date**: 2026-11-03  
**Auditor**: Builder (Autonomous Ruqa Agent)  
**Target Commit**: `040b729` (origin/main)  
**Harness**: Headless End-to-End Application Harness (`crates/ruqa-app/tests/e2e_app_harness_tests.rs`)

---

## 1. Executive Summary

A live manual run of commit `040b729` by the repository owner uncovered critical runtime defects that library-level offscreen tests failed to catch:
1. The startup HUD stayed open and its close button (`X`) did not respond to mouse clicks.
2. The Study Vault sidebar (`Ctrl+B`) showed rows that could not be expanded or collapsed via mouse, and faint pixels of the style palette bled through its top rows.
3. The startup dialog and sidebar rendered **zero text** (blank titles, blank row labels, blank button text).
4. README performance figures and screenshot were unmeasured/mocked.
5. First run logged excessive frames while idle.

To bridge the gap between isolated unit tests and live application behavior, we constructed an **End-to-End Headless Application Harness** in `ruqa-app` that exercises the exact event dispatch loop and full frame rendering pipeline into an offscreen pixmap, verifying both state transitions and pixel buffers.

The audit revealed that while geometric models, undo/redo, file persistence, and text shaping were implemented in lower crates, **several high-level interactive integrations in `ruqa-app` and rasterizers in `ruqa-render` were either stubbed with placeholder geometry or bypassed by premature event loop returns**.

---

## 2. Methodology & Harness Architecture

The test harness (`AppHarness`) instantiates a real `WhiteboardApp` instance backed by a `tiny_skia::Pixmap` of 1280×800 pixels. It dispatches synthetic events matching `winit`'s event flow:
- `handle_cursor_moved(physical_x, physical_y)`
- `handle_mouse_input(state, button)`
- `handle_mouse_wheel(delta)`
- `handle_key(key, state)`
- `render_to_pixmap(&mut pixmap)`

Assertions verify:
1. **App State**: Tool state, active selections, scene element count, viewport camera offsets, dirty flag, and modal visibility flags.
2. **Pixel Integrity**: Inspection of pixel bounding boxes ensuring shapes, icons, and text glyphs rasterize visible ink into the target framebuffer.

---

## 3. Milestone-by-Milestone Audit Matrix

| Feature / Issue | Offscreen Unit Tests | Live E2E App Reality | Audit Status | Root Cause & Evidence |
| :--- | :---: | :---: | :---: | :--- |
| **M1: Viewport Pan & Zoom (#10, #11)** | Pass | Pass | **VERIFIED** | Middle-mouse drag pans viewport camera; wheel zooms anchored at cursor. |
| **M1: Interactive Shape Drawing (#12, #14)** | Pass (in-memory) | **FAIL** | **DEFECT** | `pointer_input.rs` only handled selection and eraser; dragging with Rectangle/Ellipse/Diamond/Line/Arrow selected fell through to selection and never spawned elements! |
| **M1: Freehand Pen Drawing (#15, #16)** | Pass (unit) | **FAIL** | **DEFECT** | Freehand stroke smoothing exists in `ruqa-core`, but pointer motion in `pointer_input.rs` did not collect stroke points or commit `Element::Stroke` on mouse release. |
| **M2: Selection, Move & Resize (#19, #21, #22)** | Pass | Pass | **VERIFIED** | Marquee drag, point hit-testing, handle dragging, and element translation work once elements are in scene. |
| **M2: Delete, Duplicate, Clipboard (#51, #52)** | Pass | Pass | **VERIFIED** | Delete/Backspace removes elements; Ctrl+D duplicates; Ctrl+C / Ctrl+V copies and pastes. |
| **M2: Undo / Redo History (#23)** | Pass | Pass | **VERIFIED** | Unlimited undo/redo stack via Command pattern works across mutations. |
| **M2: .ruqa Format & Autosave (#24, #54)** | Pass | Pass | **VERIFIED** | Versioned JSON serialization and atomic tempfile-to-destination autosave work reliably. |
| **M3: Floating Toolbar (#26, #27)** | Pass | Pass | **VERIFIED** | Buttons hit-test properly; single-key shortcuts (`v`, `r`, `c`, `d`, `l`, `a`, `p`, `e`, `t`) switch tools. |
| **M3: Contextual Style Panel (#28)** | Pass | Pass | **VERIFIED** | Color swatches, stroke widths, and roughness presets update `current_style` and mutate selected elements. |
| **M3: Continuous Eraser Tool (#30)** | Pass | Pass | **VERIFIED** | Continuous drag erases touched elements into a single composite undo command. |
| **M3: Dark Theme & Canvas Grid (#31)** | Pass | Partial | **DEFECT** | Dark/Light inversion and grid dots work via `toggle_theme()`, but keyboard shortcut (`Ctrl+T`) was omitted from `keyboard.rs`. |
| **M3: Startup / Recent Files HUD (#55)** | Pass (dummy) | **FAIL** | **CRITICAL DEFECT** | 1. Early return in `handle_cursor_moved` left `cursor_pos` stale; clicking X button or outside modal never dismissed HUD.<br>2. `recent_files_render.rs` had **zero calls** to render text (only drew gray boxes). |
| **M3: Study Vault Sidebar (#56)** | Pass (dummy) | **FAIL** | **CRITICAL DEFECT** | 1. `handle_cursor_moved` skipped cursor updates when hovering sidebar; row clicks never hit-tested.<br>2. `VaultTheme.background` used alpha 252 instead of 255, letting style palette bleed through.<br>3. `vault_render.rs` drew icons but omitted all text labels (folder names, board titles). |
| **M4: PNG Export (#33, #34)** | Pass | Pass | **VERIFIED** | `Ctrl+E` and programmatic export rasterize full diagram and tight selection bounds to PNG. |
| **M4: Quit Guard Dialog (#35)** | Pass (dummy) | **DEFECT** | **DEFECT** | Dialog opens on unsaved exit, but text messages and button labels were drawn as placeholder vector bars instead of glyphs. |
| **M5: Bilingual Typography & Text (#39–#43)** | Pass | Canvas Works / UI Broken | **DEFECT** | Parley shaping and Skrifa TrueType rasterization work beautifully for canvas text, but UI chrome modules never called `render_text_element`. |

---

## 4. Root Causes Breakdown

### Defect 1: Pointer Coordinate Freezing over Modals & Overlays
In `crates/ruqa-app/src/pointer_input.rs`:
```rust
if self.show_quit_dialog
    || self.show_welcome_hud
    || (self.show_vault_sidebar && self.vault_layout().contains(screen_pt))
    || self.toolbar_pressed
    || self.style_panel_pressed
{
    return hover_changed;
}

if let Some((dx, dy)) = self.pointer.update_cursor(logical_x, logical_y) { ... }
```
When a dialog or sidebar was active, `handle_cursor_moved` returned before executing `self.pointer.update_cursor()`. Consequently, `self.pointer.cursor_pos` remained `None` or frozen at the coordinate prior to entering the UI. When `handle_mouse_input` fired, hit-testing evaluated against the stale coordinate, rendering buttons and rows unresponsive to mouse clicks.

### Defect 2: Missing UI Chrome Text Rasterization
In `recent_files_render.rs`, `vault_render.rs`, and `quit_dialog_render.rs`, render passes drew background rectangles, borders, and vector icon paths, but never invoked `ruqa_render::text_render::render_text_element`. The unit tests merely checked `non_zero_pixels > 1000`, which was satisfied by the backdrop and boxes alone.

### Defect 3: Sidebar Background Bleed-Through
In `vault_render.rs`:
```rust
background: Color::from_rgba8(20, 20, 24, 252),
```
Alpha `252` (~98.8% opacity) allowed faint pixels from underlying HUD elements (such as the style panel) to bleed through. The fill must be 100% opaque (`255`).

### Defect 4: Interactive Shape and Pen Creation Missing in Event Loop
While `demo.rs` and unit tests constructed shape primitives directly, `ruqa-app::pointer_input` lacked the mouse drag state machine for creating new rectangles, ellipses, diamonds, lines, arrows, and freehand strokes.

---

## 5. Remediation Roadmap

The defects are decomposed into five focused issues and pull requests:
1. **Issue 1 (Mouse Routing & Cursor Tracking)**: Fix `handle_cursor_moved` to update cursor position unconditionally; enable dismissing Welcome HUD via close button, Escape, Enter, or outside click; enable row expansion and board opening in Study Vault.
2. **Issue 2 (Sidebar Opacity & Theme Bleed Fix)**: Set sidebar background alpha to 255; ensure clean compositing over canvas and style panel; add `Ctrl+T` shortcut for theme toggle.
3. **Issue 3 (Chrome UI Text Rendering)**: Wire `render_text_element` into `recent_files_render`, `vault_render`, and `quit_dialog_render` to render clean labels, folder names, board titles, and button captions using bundled fonts.
4. **Issue 4 (Interactive Shape & Pen Drawing)**: Implement interactive shape drag-to-create and freehand pen stroke gathering in `ruqa-app`.
5. **Issue 5 (Redraw-on-Demand Idle Verification)**: Enforce strict `ControlFlow::Wait` idle behavior; ensure 0 frames rendered while idle; measure idle CPU usage.

