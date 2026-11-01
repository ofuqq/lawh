# ADR 0003: CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer)

- **Status**: **Accepted (Provisional until Spike 1)**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
A whiteboard application requires fluid 60+ FPS rendering of organic, hand-drawn vector paths, geometric shapes, and continuous freehand pen ink. In the Rust ecosystem, two primary 2D rendering paradigms exist:
1. **CPU Software Rasterization**: Compute pixel coverage on the CPU and blit the resulting image to the window surface (e.g. `tiny-skia` + `softbuffer`).
2. **GPU Compute Rasterization**: Execute modern 2D compute shaders via `wgpu` (e.g. Linebender’s `vello`).

We must choose an initial rendering architecture for Milestone M1 and v0.1 that maximizes hardware compatibility, avoids GPU driver friction on Linux, and keeps codebase complexity manageable for a solo founder.

---

## 2. Decision Drivers
- **Cross-Platform & Linux Reliability**: Must run on standard Linux setups (X11 & Wayland) without requiring Vulkan, OpenGL, or proprietary GPU drivers.
- **Headless CI Testing**: Must run reliably in headless GitHub Actions runners for automated pixel diffs and integration tests.
- **Dependency Cleanliness**: Must avoid version gridlock. `tiny-skia` must resolve to exactly one version in `cargo tree -d`.
- **Thin Abstraction**: No speculative generic trait hierarchies; isolate rendering cleanly in `ruqa-render`.

---

## 3. Decision Outcome & Rationale
Chosen option: **CPU-First Rendering via `tiny-skia` on CPU + `softbuffer` + stable `winit 0.30.x`**.

### Architectural Rules
1. **Core Only for `roughr`**: Depend solely on renderer-agnostic `roughr = "0.14"` core. Do NOT depend on `rough_tiny_skia` or `rough_vello` adapter crates, as they pin conflicting versions of `tiny-skia` (0.11) and `palette`.
2. **Local Path Converter**: Implement a small internal converter (~35 LOC) inside `ruqa-render` that maps `roughr::core::OpSet` directly to `tiny_skia::PathBuilder`.
3. **No Premature Caching**: Do NOT add a 256px spatial tile cache or multi-threaded `rayon` workers until the benchmark harness in Spike 1 proves single-threaded `tiny-skia` cannot maintain 60 FPS ($< 16.6\text{ ms}$) on a 5,000-element scene.
4. **Stable Windowing**: Use stable `winit 0.30.x` (avoiding the breaking 0.31 beta refactor).
5. **Presentation via `softbuffer`**: Window presentation binds directly to OS surface buffers using `softbuffer 0.4.x`.

### Provisional Gate (Spike 1 Validation)
This ADR is provisional until **Spike 1** benchmarks full-viewport cold rasterization, continuous panning, single-node drag, and idle RSS at $2560 \times 1440$ on a scene of 5,000 rectangles, 2,000 strokes, and 500 text boxes.
