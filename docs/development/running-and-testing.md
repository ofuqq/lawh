# Running and Testing Ruqa

This guide covers building, running, benchmarking, and executing automated and manual QA tests for Ruqa.

---

## 1. Verified Linux System Dependencies

Building and running the GUI window on Linux requires development libraries for X11 / Wayland, as well as testing tools for headless GUI verification:

### Ubuntu / Debian (22.04 LTS, 24.04 LTS)
```bash
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libxkbcommon-dev \
    xvfb \
    xdotool \
    imagemagick
```

### Fedora (39, 40, 41)
```bash
sudo dnf install -y \
    gcc \
    libX11-devel \
    libXcursor-devel \
    libXrandr-devel \
    libXi-devel \
    libxkbcommon-devel \
    xorg-x11-server-Xvfb \
    xdotool \
    ImageMagick
```

### Arch Linux / Manjaro
```bash
sudo pacman -S --needed \
    base-devel \
    libx11 \
    libxcursor \
    libxrandr \
    libxi \
    libxkbcommon \
    xorg-server-xvfb \
    xdotool \
    imagemagick
```

---

## 2. Running the Desktop Application

### Normal Interactive Launch (X11 or Wayland)
```bash
cargo run -p ruqa-app
```
To run with debug tracing logs:
```bash
RUST_LOG=debug cargo run -p ruqa-app
```

### Headless & CI Smoke Testing
Ruqa supports multiple execution modes depending on display availability:

1. **Headless Smoke Test (under Xvfb)**:
   Pumps $N$ synthetic frames through the real `winit` event loop and exits with status 0:
   ```bash
   xvfb-run -a cargo run -p ruqa-app -- --smoke-test 10
   ```
2. **Pure Headless Environment (No DISPLAY / No Xvfb)**:
   When neither `DISPLAY` nor `WAYLAND_DISPLAY` is exported in the environment, `ruqa-app` detects the missing display and automatically runs a headless mouse-motion event dispatch benchmark, verifying event handling without opening a window:
   ```bash
   unset DISPLAY WAYLAND_DISPLAY
   cargo run -p ruqa-app
   ```

---

## 3. Running Benchmarks & Spikes

### 2D Rasterization Benchmark (Spike 1)
Profiles `tiny-skia` CPU rasterization throughput across 50, 100, 250, 500, and 1,000 shapes at 1080p:
```bash
cargo run --release -p ruqa-render --bin spike1_bench
```

### Arabic Text Shaping & BiDi Benchmark (Spike 2)
Tests `parley` HarfRust shaping on bilingual Arabic/English strings:
```bash
cargo run --release -p ruqa-render --bin spike2_bench
```

### Rendering Microbenchmark Suite (Criterion / Divan)
Runs the comprehensive stress benchmark measuring unculled 1,000, 5,000, and 10,000 element scenes:
```bash
cargo bench -p ruqa-render -- --test
```

---

## 4. Manual QA Verification Checklist (10 Minutes)

Follow this step-by-step checklist to verify all interactive capabilities delivered across Milestones M0–M2.

| Step | Action | Keyboard / Mouse Input | Expected Behavior | Failure Indicators |
|---|---|---|---|---|
| **1** | Launch App | `cargo run -p ruqa-app` | Window opens with dark background (`#121212`), centered on screen. | Window fails to open, crashes with softbuffer error, or window is white. |
| **2** | Window Resize | Drag window border to resize | Canvas surface resizes smoothly without tearing; background remains uniform dark. | Black bands, distorted coordinates, or crash on resize. |
| **3** | Pan Canvas | Hold `Space` + Left Drag OR Middle-Click Drag | Viewport pan offsets change smoothly. *(Note: Visible motion requires shapes or grid dots on canvas).* | Window locks up, cursor stutters, or pan jumps erratically. |
| **4** | Zoom In / Out | Mouse Wheel Scroll (over canvas) | Canvas zooms smoothly anchored at the cursor position. | Zoom does not center on pointer, or canvas flips upside down. |
| **5** | Marquee Selection | Left Click + Drag on Canvas | Translucent rectangular marquee box with blue outline (`#3388ff`) tracks mouse. | Marquee does not render, or leaves stale ghost boxes on mouse release (Bug #87). |
| **6** | Single Shape Hit | Left Click directly on shape | Element is selected; blue bounding box outline with 8 resize handles appears. | Click on shape selects nothing, or clicks outside shape select it. |
| **7** | Shape Drag Move | Left Click + Drag on selected shape | Shape translates smoothly under cursor; bounding box and handles track motion. | Shape stays stationary, jumps to origin, or deselects on drag start. |
| **8** | Multi-Selection | Hold `Shift` + Left Click additional shapes | Both shapes become selected; bounding box expands to enclose all selected items. | Shift click replaces selection instead of adding to it. |
| **9** | Window Close | Click window close button (`X`) | Clean application exit with return code 0; terminal logs frame presentation count. | Application hangs, leaks resources, or exits with non-zero panic code. |
