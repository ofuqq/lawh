# Spike 1: 2D CPU Rendering Pipeline Benchmark

- **Date**: 2026-11-01
- **Status**: Completed
- **Associated ADR**: [ADR 0003: CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer)](../../adr/0003-rendering-backend.md)
- **Associated Issue**: [#3: Spike: Benchmark tiny-skia, roughr, and softbuffer at 60 FPS](https://github.com/ofuqq/ruqa/issues/3)

---

## 1. Objective

Validate the feasibility of CPU-only 2D software rasterization using `tiny-skia` and `roughr` at 60 FPS ($\le 16.667\text{ ms}$ per frame) on standard developer hardware without GPU acceleration.

Specifically, this spike measures:
1. Cold vs steady-state rasterization time across varying element counts.
2. Latency percentiles ($p50$, $p95$, $\max$) on a full $1920 \times 1080$ 32-bit RGBA pixmap.
3. Memory footprint (Resident Set Size, RSS) under load.
4. Bottleneck breakdown between `roughr` geometry generation and `tiny-skia` scanline rasterization.

---

## 2. Test Environment

- **CPU**: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz (4 physical cores, 8 threads, Turbo up to 3.40GHz)
- **RAM**: 15 GiB DDR4
- **OS**: Linux (kernel 7.2.11-arch1-1 x86_64)
- **Rust Toolchain**: `rustc 1.91.0`, Cargo profile: `release` (opt-level = 3)
- **Canvas Resolution**: $1920 \times 1080$ pixels ($8.29\text{ MB}$ uncompressed 32-bit RGBA)
- **Scene Composition**: Even mixture of hand-drawn rectangles, ellipses, and multi-point freehand strokes with anti-aliasing enabled.

---

## 3. Benchmark Results

### Scaling Profile (Headless $1920 \times 1080$)

| Element Count | Path Count | Gen Time | Frames | Min (ms) | Mean (ms) | Median / p50 (ms) | p95 (ms) | Max (ms) | Calculated FPS | Process RSS | Target Met ($\le 16.6\text{ms}$) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **50** | 50 | 0.20 ms | 100 | 4.690 | **5.142** | 5.035 | 5.892 | 6.570 | **194.5 FPS** | 11.36 MB | **YES** |
| **100** | 100 | 0.33 ms | 100 | 9.252 | **10.584** | 10.061 | 14.857 | 17.647 | **94.5 FPS** | 11.38 MB | **YES** |
| **250** | 250 | 0.87 ms | 100 | 23.335 | **24.790** | 24.412 | 28.472 | 33.466 | **40.3 FPS** | 11.50 MB | No |
| **500** | 500 | 3.47 ms | 50 | 46.191 | **49.882** | 48.902 | 57.522 | 74.965 | **20.0 FPS** | 11.73 MB | No |
| **1000** | 1000 | 3.52 ms | 50 | 92.840 | **102.144** | 98.738 | 121.770 | 153.614 | **9.8 FPS** | 12.15 MB | No |

---

## 4. Key Findings & Bottleneck Analysis

1. **Roughr Geometry Generation is Fast**:
   Generating synthetic rough geometry for 1,000 shapes takes only **3.52 ms** total ($\approx 3.5\,\mu\text{s}$ per shape). Shape generation is not the bottleneck.

2. **Rasterization Scales Linearly with Visible Elements**:
   Rasterization takes approximately **$0.10\text{ ms}$ per visible rough element** on single-threaded CPU anti-aliasing.
   - For typical visible viewports ($\le 100$ shapes simultaneously on screen), frame times stay well within budget: **$5.1\text{ ms}$ (194 FPS)** at 50 elements and **$10.5\text{ ms}$ (94 FPS)** at 100 elements.
   - Without viewport culling, rendering 1,000 offscreen elements degrades performance to $102\text{ ms}$ (9.8 FPS).

3. **Memory Footprint is Extremely Lean**:
   Total process Resident Set Size (RSS) remained below **$12.2\text{ MB}$** throughout all benchmark runs (including the $8.3\text{ MB}$ frame buffer).

---

## 5. Architectural Recommendations & Mitigations

1. **Accept ADR 0003**:
   `tiny-skia` + `softbuffer` is viable and approved as the primary rendering engine for v0.1.0 MVP. It eliminates GPU driver bugs, avoids `wgpu` dependency bloat, and runs reliably in headless CI and on budget Linux hardware.

2. **Mandatory Viewport Culling ([#30](https://github.com/ofuqq/ruqa/issues/30))**:
   The canvas scene graph must query an R-tree or spatial bounding box grid to discard elements outside the current window viewport before rasterization. This guarantees that frame render passes only draw the $\approx 50\text{--}150$ visible elements, preserving $\ge 60\text{ FPS}$ even when documents contain thousands of items.

3. **Cached Path Representation**:
   Render nodes should cache their converted `tiny_skia::Path` instances and transform matrices so that panning does not re-invoke `roughr` geometry generators or stroke calculations.

4. **Dirty Region Tracking & Incremental Blits**:
   During active tool interactions (such as drawing a single freehand stroke or dragging a selection handle), redraw only the damaged sub-rectangles rather than blitting the full $1920 \times 1080$ frame.
