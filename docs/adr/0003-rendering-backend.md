# ADR 0003: CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer)

- **Status**: **Accepted**
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
3. **Viewport Culling as First Defense**: Maintain bounding boxes in the scene graph to cull offscreen items prior to rasterization ([#30](https://github.com/ofuqq/ruqa/issues/30)).
4. **Stable Windowing**: Use stable `winit 0.30.x` (avoiding the breaking 0.31 beta refactor).
5. **Presentation via `softbuffer`**: Window presentation binds directly to OS surface buffers using `softbuffer 0.4.x`.

### Validation Evidence (Spike 1 Benchmark)
On 2026-11-01, Spike 1 ([report](../research/spikes/spike-1-rendering.md)) profiled headless software rasterization on an Intel i5-8250U CPU across 1920x1080 32-bit RGBA frames:
- **50 elements**: $5.14\text{ ms}$ ($194.5\text{ FPS}$), RSS $11.36\text{ MB}$ — **Target met**.
- **100 elements**: $10.58\text{ ms}$ ($94.5\text{ FPS}$), RSS $11.38\text{ MB}$ — **Target met**.
- **Path generation time**: $< 3.6\text{ ms}$ for 1,000 shapes ($\approx 3.5\,\mu\text{s}$ per shape).
- **Process memory footprint**: $< 12.2\text{ MB}$ total RSS under full load.

The results confirm that for active viewport densities ($\le 100$ visible elements), CPU rasterization delivers $\ge 90\text{ FPS}$ with negligible memory overhead. Pairing `tiny-skia` with spatial viewport culling guarantees 60 FPS performance on infinite canvases without GPU complexity.
