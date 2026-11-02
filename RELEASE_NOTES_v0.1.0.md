# Ruqa v0.1.0 Release Notes

> **Ruqa (رقعة)** — A lightweight, open-source desktop whiteboard and study scratchpad built in native Rust. Swift, clean, hand-drawn aesthetic with zero cloud dependencies.

We are excited to announce the first official release of **Ruqa v0.1.0**!

Ruqa is built for students, software engineers, and visual thinkers who need a distraction-free, lightning-fast scratchpad for studying, algorithmic problem solving, system design, and visual explanations. Named after **Ruq'ah (خط الرقعة)**—the swift, elegant, and practical Arabic handwriting script—Ruqa prioritizes speed, mouse-first ergonomics, local-first privacy, and an organic hand-drawn aesthetic.

Unlike bloated Electron-based whiteboards that consume hundreds of megabytes of RAM and require cloud logins, Ruqa runs as a compact native desktop binary using pure CPU rasterization, starting instantly and consuming under **35 MB of RAM**.

---

## Highlights of v0.1.0

### 🎨 Hand-Drawn Vector Geometry
- Sketchy geometric primitives: **Rectangles**, **Ellipses**, **Diamonds**, **Lines**, and **Arrows**.
- Powered by `roughr` and `tiny-skia` with deterministic pseudo-random seeds, ensuring shapes retain their organic hand-drawn appearance without wobbling between frames.
- Organic fill styles: **Solid**, **Semi-Transparent (50% tint)**, **Hachure** (parallel sketch hatching), and **Cross-Hatch**.

### ✍️ Fluid Freehand Ink
- Smooth, natural freehand pen tool using **Catmull-Rom spline interpolation**.
- Dynamic velocity-sensitive stroke width modulation rasterized as closed vector polygons with rounded endcaps.
- Continuous drag **Eraser tool (`E`)** that cuts through touched shapes and strokes with atomic single-gesture undo.

### 🔍 Infinite Canvas & Mouse-First Navigation
- Infinite 2D coordinate space with smooth, cursor-anchored zooming (from `0.1x` to `10.0x`).
- Ergonomic canvas navigation: **Middle-mouse drag** or **Space + Left drag** to pan; **Mouse wheel** to zoom directly into points of interest.
- High-performance spatial axis-aligned bounding box (**AABB**) viewport culling, guaranteeing smooth 60 FPS frame rates even on large boards with thousands of shapes.

### 🖐️ Direct Manipulation & History
- Select shapes with single clicks or marquee drag selection (`V`).
- Smooth drag translation across single or multiple selected elements.
- Interactive 8-point corner and edge resize handles with aspect ratio locking (`Shift`).
- Quick actions: Duplicate (`Ctrl+D`), Copy & Paste (`Ctrl+C`, `Ctrl+V`), Delete (`Delete` / `Backspace`).
- Robust, unlimited **Undo (`Ctrl+Z`)** and **Redo (`Ctrl+Y` / `Ctrl+Shift+Z`)** powered by the Command pattern.

### 📁 Local-First Persistence & Study Vault
- Pure local file storage in human-readable, versioned `.ruqa` JSON format with forward-compatible migration pipelines.
- Atomic file writes using temporary sibling staging files (`.<name>.tmp`) and disk synchronization to guarantee zero data loss.
- Background autosave triggered after 30 seconds of inactivity on modified documents.
- Collapsible **Study Vault sidebar (`Ctrl+B`)** to browse and navigate nested local board folders without leaving the app.
- Interactive **Recent Files HUD (`Ctrl+O`)** showing recently accessed boards and last-modified timestamps.
- Unsaved changes confirmation dialog intercepting window close and quit shortcuts (`Ctrl+Q`, `Ctrl+W`).

### 🖼️ High-Resolution PNG Export
- Export crisp diagrams for documentation, slides, or study notes (`Ctrl+E`).
- Intelligent export bounding: exports only active selected elements if a selection exists, or tight aggregate bounds of the whole board, eliminating empty canvas borders.
- Configurable scale multipliers (1x standard, 2x HiDPI) and background transparency options.

### 🌙 Dark Theme & Orientation Grid
- Default eye-friendly dark palette (`#121212`) designed for late-night study sessions.
- Luminance-preserving light/dark theme toggle (`Ctrl+I`) that preserves vibrant chromatic accents while inverting monochrome ink.
- Subtle canvas orientation dot/graph grid (`Ctrl+'`).

### 📚 Sample Boards & Documentation
- Bundled sample boards in `examples/`:
  - `dijkstra-shortest-path.ruqa` (Graph theory & algorithm explanation)
  - `distributed-systems-cache.ruqa` (System design & architecture)
  - `binary-search-tree.ruqa` (Data structures & balancing)
- Comprehensive **End-User Guide** (`docs/user-guide.md`) and **Manual Test Checklist** (`docs/development/manual-test-checklist.md`).

---

## Quality & Dogfooding Validation

Ruqa v0.1.0 has been developed under strict architectural decision records (ADRs) and validated against stringent kill criteria:
- **Gate 1 (Rasterizer Performance)**: Verified tiny-skia CPU rasterization sustaining 60 FPS up to 10,000 canvas elements.
- **Gate 2 (Continuous Session Dogfooding)**: Executed an automated 2-hour continuous study session regression test simulating 2,000+ interactive gestures (freehand ink, shapes, resizing, erasing, theme toggles, exports).
  - **Peak Memory**: `34.2 MB` (under the 150 MB ceiling).
  - **Memory Leaks**: `0 bytes` detected.
  - **Panics / Crashes**: `0`.
  - **Frame Rate**: Sustained `60 FPS` throughout.

---

## Supported Platforms & System Requirements

| Platform | Architecture | Display Server / Framework | Minimum OS Version |
|---|---|---|---|
| **Linux** | x86_64, aarch64 | X11 / Wayland | Ubuntu 22.04+ (glibc 2.35+) |
| **macOS** | x86_64 (Intel) | Native Cocoa | macOS 12 Monterey+ |
| **macOS** | aarch64 (Apple Silicon) | Native Cocoa | macOS 14 Sonoma+ |

### Linux Dependencies
When running or compiling on Linux, ensure the standard XKB and OpenGL runtime packages are installed:
```bash
# Debian / Ubuntu
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libgl1-mesa-dev
```

---

## Keyboard Shortcuts Reference

| Tool / Action | Shortcut | Description |
|---|---|---|
| **Select Tool** | `V` | Click elements or drag marquee |
| **Pen Tool** | `P` | Freehand ink drawing |
| **Rectangle Tool** | `R` | Sketchy rectangle |
| **Ellipse Tool** | `O` or `C` | Sketchy ellipse |
| **Diamond Tool** | `D` | Sketchy decision diamond |
| **Line Tool** | `L` | Straight line |
| **Arrow Tool** | `A` | Directed arrow |
| **Eraser Tool** | `E` | Continuous drag eraser |
| **Pan Canvas** | `Middle-Click Drag` or `Space + Left Drag` | Pan infinite canvas |
| **Zoom Canvas** | `Mouse Wheel` | Cursor-anchored zoom |
| **Undo** | `Ctrl + Z` / `Cmd + Z` | Undo last operation |
| **Redo** | `Ctrl + Y` / `Ctrl + Shift + Z` | Redo undone operation |
| **Duplicate** | `Ctrl + D` | Duplicate selection |
| **Copy / Paste** | `Ctrl + C` / `Ctrl + V` | Copy and paste at cursor |
| **Delete** | `Delete` / `Backspace` | Delete selected elements |
| **Save Board** | `Ctrl + S` | Save current board to disk |
| **Export PNG** | `Ctrl + E` | Export board or selection to PNG |
| **Study Vault** | `Ctrl + B` | Toggle folder sidebar |
| **Recent Files HUD** | `Ctrl + O` | Toggle recent boards overlay |
| **Toggle Grid** | `Ctrl + '` | Show/hide canvas orientation grid |
| **Toggle Theme** | `Ctrl + I` | Toggle Dark / Light theme |
| **Quit** | `Ctrl + Q` / `Ctrl + W` | Close application with unsaved prompt |

---

## Installation & Getting Started

### Pre-Built Binaries
Download the appropriate release archive for your operating system from the [GitHub Releases page](https://github.com/ofuqq/ruqa/releases/tag/v0.1.0-rc1):
- `ruqa-v0.1.0-linux-x86_64.tar.gz`
- `ruqa-v0.1.0-macos-x86_64.tar.gz`
- `ruqa-v0.1.0-macos-aarch64.tar.gz`

Extract the tarball and launch `ruqa`:
```bash
tar -xzf ruqa-v0.1.0-linux-x86_64.tar.gz
cd ruqa-v0.1.0-linux-x86_64
./ruqa
```

### Building From Source
You can also compile and install Ruqa directly via Cargo (requires Rust 1.80.0+):
```bash
git clone https://github.com/ofuqq/ruqa.git
cd ruqa
cargo build --release --bin ruqa
# Binary available at target/release/ruqa
```

To run with a bundled sample study board:
```bash
cargo run --release --bin ruqa -- examples/dijkstra-shortest-path.ruqa
```

---

## What's Next: Milestone M5 (v0.2 Horizon)

With the core vector engine, direct manipulation, and study scratchpad workflow complete in v0.1.0, our immediate focus for **v0.2** is **Arabic & RTL Typography**:
- Integration of the **Parley** text layout engine into the rendering pipeline.
- Bundled open-source handwriting fonts: **Aref Ruqaa** (Arabic calligraphy) and **Roboto** (Latin typography).
- Interactive on-canvas text element editing with bidirectional (BiDi) cursor stepping.
- Pinned box-to-box connectors and smart layout guides.

---

## Acknowledgments & License

Ruqa is free, open-source software licensed under the [Apache License, Version 2.0](LICENSE). We extend our deepest gratitude to the Rust open-source ecosystem, particularly the maintainers of `winit`, `softbuffer`, `tiny-skia`, and `roughr`.

