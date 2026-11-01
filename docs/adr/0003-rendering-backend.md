# ADR 0003: CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer)

- **Status**: **Proposed** (Pending workspace dependency resolution verification and M1 benchmark data)
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
A whiteboard application requires fluid 60+ FPS rendering of organic, hand-drawn vector paths, geometric shapes, and continuous freehand pen ink. In the Rust ecosystem, two primary 2D rendering paradigms exist:
1. **CPU Software Rasterization**: Compute pixel coverage on the CPU and blit the resulting image to the window surface (e.g., `tiny-skia` + `softbuffer`).
2. **GPU Compute Rasterization**: Execute modern 2D compute shaders via `wgpu` (e.g., Linebender’s `vello`).

We must choose an initial rendering architecture for Milestone M1 and v0.1 that maximizes hardware compatibility, avoids GPU driver friction on Linux, and keeps codebase complexity manageable for a solo founder.

---

## 2. Decision Drivers
- **Cross-Platform & Linux Reliability**: Must run on standard Linux setups (X11 & Wayland) without requiring Vulkan, OpenGL, or proprietary GPU drivers.
- **Headless CI Testing**: Must run reliably in headless GitHub Actions runners for automated pixel diffs and integration tests.
- **Ecosystem Maturity**: Active crate maintenance, proven production stability, and clear documentation.
- **Thin Abstraction**: No speculative generic trait hierarchies; isolate rendering cleanly in a dedicated crate.
- **Single Crate Resolution**: `tiny-skia`, `rough_tiny_skia`, and `roughr` must resolve to a single unified version of `tiny-skia` without duplicate version bloat.

---

## 3. Verified Dependency Constraints (crates.io Audit)
- `tiny-skia`: 0.12.0 is latest, but `rough_tiny_skia` (0.14.0) explicitly requires `tiny-skia = "^0.11"`. Therefore, pinning the workspace to `tiny-skia = "0.11.4"` ensures that `rough_tiny_skia`, `roughr`, and `lawh` resolve to **one unified `tiny-skia` version**.
- `winit`: Use **stable `0.30.x`** (e.g., `0.30.9` or `0.30.13`), avoiding the `0.31.0-beta.x` line which has ongoing breaking event-loop refactors.
- `softbuffer`: Version `0.4.8` provides cross-platform software presentation, binding native window handles directly to a 32-bit presentation buffer.
- `rough_tiny_skia` (v0.14) & `roughr` (v0.14): Single-maintainer crates with moderate download volume (~7.9k / ~46k). Acknowledged as a project risk, mitigated by isolating their use inside `lawh-render`.

---

## 4. How Pixels Reach the Window (`softbuffer`)
1. `winit` (0.30.x) creates the OS window.
2. `softbuffer::Context` and `softbuffer::Surface` wrap the window handle.
3. On every frame, `tiny-skia` renders vector paths into an in-memory `tiny_skia::PixmapMut`.
4. The RGBA bytes from `PixmapMut` are copied directly into `surface.buffer_mut()`, and `buffer.present()` flips the frame to the screen via X11/Wayland shared memory.

---

## 5. Decision Outcome & Rationale
Proposed option: **CPU-First Rendering via `tiny-skia` (0.11.x) + `softbuffer` (0.4.x) + `winit` (0.30.x)**.

### Rationale
- **Universal Portability**: Guaranteed to work on virtual machines, low-end laptops, and headless CI runners with zero GPU hardware configuration.
- **High Performance for MVP**: `tiny-skia` achieves $> 60\text{ FPS}$ on boards with hundreds of shapes using SIMD optimizations.
- **Thin Isolation**: Rendering logic is confined to `crates/lawh-render`. The core domain model (`lawh-core`) remains completely decoupled from the rasterizer.

### Performance Benchmark Gate (Decision 7 Exit Criterion)
This ADR will be formally transitioned to **Accepted** only after Milestone M1 implements an automated headless benchmark testing frame render times at **1,000, 5,000, and 10,000 strokes**:
- **Target**: Frame render time $< 16.6\text{ ms}$ (sustained 60 FPS) at 1,000 strokes.
- If benchmarks show CPU bottlenecking on realistic study diagrams, we will use the data to design a clean migration to `vello` (GPU) for post-v0.1.
