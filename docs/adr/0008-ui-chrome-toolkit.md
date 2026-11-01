# ADR 0008: UI Chrome & Toolbar Toolkit Selection

- **Status**: **Proposed**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Mirsam is primarily an infinite canvas, but requires a clean, minimal UI chrome:
- Top tool selection bar (Select, Pen, Rectangle, Ellipse, Arrow, Text, Eraser).
- Contextual property panel (stroke width, stroke color, fill style, rough roughness).
- Zoom level indicator, undo/redo triggers, and settings modal.

The choice of UI chrome toolkit must balance rendering performance, binary size, dependency footprint, and ease of embedding over a software-rasterized canvas.

---

## 2. Considered Options

### Option 1: `egui` Overlay
- **Pros**: Mature immediate-mode GUI with massive ecosystem; easily renders over existing frames; fast to prototype.
- **Cons**: Requires either software-rasterizing egui tessellated meshes via a CPU rasterizer or managing a hybrid GPU/CPU pipeline; high dependency weight; default styling requires substantial customization to fit a sleek dark theme.

### Option 2: Custom Minimal CPU Chrome (`mirsam-render` / `mirsam-app`)
- **Pros**: Zero additional GUI framework dependencies; drawn directly into the `tiny-skia` pixel buffer via standard 2D rectangles, icons, and text; 100% control over look, feel, and hit-testing; zero impedance mismatch.
- **Cons**: Requires writing simple button hit-testing and dropdown menus manually (~200–400 LOC).

### Option 3: Lightweight Retained/Immediate Widget Micro-layer
- Using small modular libraries (e.g. `tiny-skia-ui` or custom dispatchers).

---

## 3. Decision Recommendation & Next Steps

**Status**: **Proposed**.
Decision is formally deferred to the conclusion of **Spike 1** and **Spike 2**:
- If `mirsam-render`'s CPU drawing pipeline makes drawing minimal UI buttons and toolbars trivial (< 300 LOC), Option 2 is preferred for zero bloat.
- If rich floating modals, color pickers, or font selectors prove too complex to maintain manually, Option 1 (`egui`) will be evaluated.
