# ADR 0003: CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer)

- **Status**: **Accepted (with Culling Condition)**
- **Date**: 2026-11-01 (Updated 2026-11-02)
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
3. **Viewport Culling as First Defense**: Maintain bounding boxes in the scene graph to cull offscreen items prior to rasterization ([#32](https://github.com/ofuqq/ruqa/issues/32)).
4. **Stable Windowing**: Use stable `winit 0.30.x` (avoiding breaking refactors).
5. **Presentation via `softbuffer`**: Window presentation binds directly to OS surface buffers using `softbuffer 0.4.x`.

### Validation Evidence (Spike 1 & Microbenchmarks)
Empirical benchmarks run on an Intel Core i5-8250U CPU @ 1.60GHz (4 cores / 8 threads, 15.4 GiB RAM, Arch Linux x86_64) across 1920x1080 32-bit RGBA frames:

| Visible Elements | Mean Frame Latency | Effective FPS | RSS Memory | Target Status |
|---|---|---|---|---|
| **50 elements** | $8.62\text{ ms}$ | $116.0\text{ FPS}$ | $11.34\text{ MB}$ | **Target Met** ($< 16.67\text{ ms}$) |
| **100 elements** | $14.79\text{ ms}$ | $67.6\text{ FPS}$ | $11.36\text{ MB}$ | **Target Met** ($< 16.67\text{ ms}$) |
| **250 elements** | $28.94\text{ ms}$ | $34.6\text{ FPS}$ | $11.40\text{ MB}$ | Failed 60 FPS ($> 16.67\text{ ms}$) |
| **500 elements** | $71.82\text{ ms}$ | $13.9\text{ FPS}$ | $11.45\text{ MB}$ | Failed 60 FPS |
| **1,000 elements** | $113.52\text{ ms}$ | $8.8\text{ FPS}$ | $11.52\text{ MB}$ | Failed 60 FPS |
| **5,000 elements** | $522.43\text{ ms}$ | $1.9\text{ FPS}$ | $12.10\text{ MB}$ | Unculled Stress Benchmark |
| **10,000 elements** | $1,118.99\text{ ms}$ | $0.9\text{ FPS}$ | $13.25\text{ MB}$ | Unculled Stress Benchmark |

### Condition for Acceptance
Single-threaded `tiny-skia` comfortably sustains $\ge 60\text{ FPS}$ for scenes up to $\approx 100$ simultaneously visible elements on modest laptop hardware with minimal RAM ($< 12\text{ MB}$).

However, without viewport culling, rendering latency scales linearly with scene element count ($O(N)$), degrading below 15 FPS beyond 250 elements and down to 1.9 FPS at 5,000 elements.

**Mandatory Condition**: Spatial viewport culling ([#32](https://github.com/ofuqq/ruqa/issues/32)) must be active in v0.1.0 so that only elements intersecting the visible viewport camera are passed to `tiny-skia`. Dynamic tile caching and `rayon` thread pools are deferred post-v0.1.0 until benchmarks demonstrate scenes exceeding 1,000 *visible* viewport elements.
