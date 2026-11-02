# ADR 0008: UI Chrome & Toolbar Toolkit Selection

- **Status**: **Accepted (Option 2: Custom Minimal CPU Chrome)**
- **Date**: 2026-11-01 (Decided 2026-11-02)
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Ruqa is primarily an infinite whiteboard canvas, but requires a clean, minimal UI chrome:
- Top tool selection bar (Select `V`, Pen `P`, Rectangle `R`, Ellipse `O`, Diamond `D`, Line `L`, Arrow `A`, Text `T`, Eraser `E`).
- Contextual property panel (stroke width, stroke color, fill style).
- Study vault folder sidebar (`Ctrl+B`) for board switching.
- Zoom level indicator and undo/redo triggers.

The choice of UI chrome toolkit must balance rendering performance, binary size, dependency footprint, and maintain the zero-GPU-dependency requirement.

---

## 2. Considered Options

### Option 1: `egui` Overlay
- **Pros**: Mature immediate-mode GUI with extensive widgets; fast to prototype.
- **Cons**:
  - Requires GPU pipeline (`wgpu` / `glow`) or software mesh rasterization (`egui_tiny_skia`). A GPU pipeline violates Ruqa's zero-GPU-dependency guarantee; software mesh rasterization introduces unnecessary CPU triangle tessellation overhead.
  - Inflates compile time and binary footprint with hundreds of unused widgets.
  - Conflicts with Ruqa's custom dark aesthetic without extensive theme overrides.

### Option 2: Custom Minimal CPU Chrome (`ruqa-render` / `ruqa-app`)
- **Pros**:
  - **Zero New Dependencies**: Reuses `tiny-skia` primitives (rounded rects, paths) and `parley` text already in the workspace.
  - **Hardware Neutrality**: Renders cleanly on CPU via `softbuffer` across any standard Linux (X11/Wayland), Windows, or macOS environment.
  - **Negligible Footprint**: Maintains Ruqa's tiny memory profile ($< 12\text{ MB}$ RSS).
  - **Simplicity**: Ruqa’s UI surface is intentionally tiny (a floating HUD pill, tool buttons, color badges, and sidebar list). Total UI rendering and hit-testing code is estimated under ~350 LOC, matching our modular file cap.
- **Cons**: Requires implementing basic hover states and button hit-testing manually.

### Option 3: Lightweight Retained/Immediate Widget Micro-layer
- Third-party micro-toolkits (e.g. `tiny-skia-ui`).
- **Cons**: Unmaintained, adds third-party supply chain risk.

---

## 3. Decision Outcome & Rationale

We select **Option 2: Custom Minimal CPU Chrome**.

### Decision Drivers & Rationale:
1. **Zero-GPU Architecture**: Ruqa's core value proposition is zero-friction execution on low-spec Linux laptops, VMs, and older educational hardware without Vulkan, OpenGL, or proprietary GPU drivers. `egui` would force GPU initialization or an awkward software triangle tessellator.
2. **Memory Footprint**: Measured baseline RSS in Spike 1 is $11.34\text{ MB}$. Custom CPU chrome introduces zero heap allocation per frame and maintains this sub-15 MB footprint.
3. **Product Scope**: Ruqa's UI chrome is strictly minimalist by design (no heavy data grids, tree views, or docked panels). Drawing a floating HUD toolbar and folder sidebar directly into the `tiny-skia` pixmap keeps the codebase clean, readable, and manageable for a solo developer.
4. **Hit-Testing Simplicity**: Bounding-box and button hit-testing use existing `ruqa-core::geometry::Rect::contains(point)` logic with zero impedance mismatch.
