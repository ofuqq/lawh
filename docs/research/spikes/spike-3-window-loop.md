# Spike 3: Winit 0.30 Event Loop & Mouse Dispatch

- **Date**: 2026-11-01
- **Status**: Completed (Validation Successful)
- **Associated Issue**: [#5: Spike: Validate winit 0.30 event loop and mouse dispatch](https://github.com/ofuqq/ruqa/issues/5)
- **Associated Milestone**: [M0: Foundation, Spikes & Testing Harness](../../roadmap.md)

---

## 1. Objective

Validate the `winit 0.30` `ApplicationHandler` event loop architecture and mouse-first interaction dispatch for the Ruqa whiteboard on Linux (X11 & Wayland), focusing on:
1. High-frequency pointer motion handling without heap allocation churn.
2. Middle-click and Space+drag canvas panning.
3. Anchored mouse wheel zoom centered at cursor coordinates.
4. Window resizing and buffer surface dimension updates without crashes or tearing.
5. Overall evaluation of **Gate 1 (Day-14 Spike Gate)** across Spikes 1, 2, and 3.

---

## 2. Test Environment

- **CPU**: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz (8 threads)
- **RAM**: 15 GiB DDR4
- **OS**: Linux 7.2.11-arch1-1 x86_64
- **Windowing Stack**: `winit = "0.30.13"`, `softbuffer = "0.4.8"`, `x11rb = "0.13"`, `smithay-client-toolkit = "0.19"`
- **Rust Toolchain**: `rustc 1.91.0`, profile: `release`

---

## 3. High-Frequency Pointer Benchmark Results

We simulated 10,000 continuous mouse motion events with alternating middle-click drag pan gestures and mouse wheel zoom events:

| Metric | Measured Value | Budget Target | Status |
|:---|:---:|:---:|:---:|
| **Total Events Processed** | **10,000 events** | — | — |
| **Total Duration** | **0.651 ms** | — | — |
| **Average Latency Per Event** | **65.1 ns** | $< 1,000\text{ ns}$ ($1\,\mu\text{s}$) | **PASSED** |
| **Heap Allocations During Motion** | **0 bytes** (Zero alloc) | 0 allocations | **PASSED** |

### Key Technical Findings:
1. **Zero-Allocation Motion Dispatch**:
   `PointerState` tracks coordinates in fixed value types (`Option<(f64, f64)>` and `bool` flags) with zero heap allocation. Even at 1,000 Hz or 8,000 Hz polling rates, event dispatch consumes only **$65.1\text{ ns}$ per event**, avoiding GC pauses and frame stutter.

2. **Anchored Zoom Invariant**:
   The anchored zoom formula ($x_{\text{canvas}} = (x_{\text{screen}} - \text{offset}_x) / \text{zoom}$) preserves the exact canvas world coordinates under the cursor across consecutive zoom factor scaling changes.

3. **Clean Window Resizing**:
   `WindowEvent::Resized` safely guards against zero width/height minimized states using `NonZeroU32::new()`, immediately updating the `softbuffer::Surface` without tearing or panic.

---

## 4. Evaluation of Gate 1 (Day-14 Spike Gate)

Per [Kill Criteria](../../product/kill-criteria.md), Gate 1 requires passing all three technical spikes before proceeding into Milestone M1:

| Spike | Target Criteria | Result | Verdict |
|:---|:---|:---|:---:|
| **Spike 1: 2D CPU Rendering** ([#3](https://github.com/ofuqq/ruqa/issues/3)) | $\ge 60\text{ FPS}$ ($\le 16.6\text{ ms}$) on typical viewport; memory $< 20\text{ MB}$ | **$5.1\text{ ms}$ (195 FPS)** at 50 elements; **$10.6\text{ ms}$ (95 FPS)** at 100 elements; RSS **$11.4\text{ MB}$**. Viewport culling ([#30](https://github.com/ofuqq/ruqa/issues/30)) mitigates offscreen scenes. | **PASSED** (ADR 0003 Accepted) |
| **Spike 2: Bilingual Shaping** ([#4](https://github.com/ofuqq/ruqa/issues/4)) | Connect Arabic cursive letters; isolate Latin tokens; latency $< 5\text{ ms}$ | **$1.77\text{ ms}$** mean paragraph shaping; connected ligatures; pure LTR isolation for math/code. Zero system C library dependencies. | **PASSED** (ADR 0007 Accepted) |
| **Spike 3: Winit 0.30 Loop** ([#5](https://github.com/ofuqq/ruqa/issues/5)) | Zero-allocation pointer dispatch; middle-drag pan; anchored zoom | **$65.1\text{ ns}$** per motion event; zero heap allocations; stable resize handling. | **PASSED** |

### Gate 1 Decision:
**ALL THREE TECHNICAL SPIKES PASSED.**
Contingency ADR 0006 (Tauri fallback) is **NOT triggered**. Ruqa continues with full confidence on its native Rust 3-crate architecture (`ruqa-core`, `ruqa-render`, `ruqa-app`).
