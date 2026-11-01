# Competitive Analysis & Strategic Positioning

To build a differentiated tool, Ruqa learns from existing digital drawing and whiteboard tools while maintaining a strict focus on a mouse-first, local-first study scratchpad with bilingual typography.

---

## 1. Feature & Positioning Comparison

| Feature / Dimension | **Ruqa** (Our Project) | **Excalidraw** | **drafft-ink** | **tldraw** | **Obsidian Canvas** | **Rnote** | **Lorien** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Primary Focus** | Solo study vault, local explanation | General sketch & web collab | Multi-user whiteboard & LaTeX math | Canvas SDK for web apps | Note-taking & card linking | Stylus handwritten notebook | Freehand infinite sketchpad |
| **Core Technology** | Rust (Native Desktop) | TypeScript / React / Canvas | Rust / WebGPU (vello) | TypeScript / React / Canvas | Electron / TypeScript | Rust / GTK4 / Cairo | Godot Engine / GDScript |
| **License** | **Apache-2.0** | **MIT** | **AGPL-3.0** | **Custom Proprietary SDK** | **Proprietary App** (Open JSON spec) | **GPL-3.0-or-later** | **MIT** |
| **Aesthetic Style** | Hand-drawn rough sketch | Hand-drawn rough sketch | Hand-drawn rough sketch | Modern clean vector/flat | Standard box-and-card | Stylus brush strokes & shapes | Smooth vector ink |
| **Input Focus** | **Mouse & Keyboard First** | Mouse / Keyboard / Touch | Mouse / Keyboard | Mouse / Touch | Mouse / Keyboard | **Stylus / Tablet First** | **Stylus / Tablet First** |
| **Offline Architecture** | 100% Native binary | Browser / Electron wrapper | Native / Web container | Browser / Web SDK | Electron desktop app | Native Linux / Flatpak | Native desktop binary |
| **Memory Footprint** | **Target: < 100 MB** | ~150–350 MB | ~100–250 MB | ~150–400 MB | ~250–600 MB | ~80–180 MB | ~50–120 MB |
| **Multi-Board Study Vault** | **Native Folder Sidebar + Search** | None (Single board tabs) | None (Single board rooms) | None (Web boards) | Deep Vault integration | File browser integration | None |
| **Arabic / RTL Support** | **First-Class (Bundled Fonts)** | Incomplete (UI & BiDi bugs) | Basic (via parley) | Basic browser BiDi | Standard Markdown BiDi | Limited Cairo BiDi | None (No text tool) |
| **Real-time Collaboration** | **None (Explicit Non-Goal)** | Full via WebSockets | Full via Loro CRDT | Full via Liveblocks | None (Local vault) | None | None |

---

## 2. In-Depth Competitor Breakdown

### Excalidraw
- **Strengths**: The gold standard for hand-drawn sketch aesthetics. Huge community, permissive MIT license, rich shape library.
- **Weaknesses**: Browser-first. Lags severely past 5,000 elements (Issue #7280). Persistent Arabic/RTL issues: lacks bundled Arabic sketchy fonts (Issue #8394) and suffers from recurring RTL UI regressions (Issues #12362, #12387, #12404). Multi-board organization is absent.
- **Ruqa's Differentiation**: Native desktop speed (<100 MB RAM), bundled Arabic hand-drawn fonts (Aref Ruqaa and Vazirmatn), and a built-in Study Vault folder sidebar.

### drafft-ink
- **Strengths**: Highly capable open-source Rust infinite canvas created by Patrick Wieschollek in early 2026. Integrates vello, parley, egui, roughr, smart alignment snapping, and inline LaTeX math.
- **Licensing & Architecture Implications**: Licensed under **AGPL-3.0**. Because Ruqa has adopted the permissive Apache-2.0 license, we **cannot fork or copy code from drafft-ink**. We maintain a strict clean-room implementation. Furthermore, drafft-ink requires WebGPU compute shaders, lacks multi-board study vault management, and focuses on multi-user CRDT synchronization.
- **Ruqa's Differentiation**: Permissive Apache-2.0 license, CPU-first software rasterization without WebGPU hardware requirements, and dedicated solo study vault tools.

### Rnote
- **Strengths**: Mature open-source Rust drawing application (11.7k GitHub stars in 2026). Excellent stylus input, pressure smoothing, and GNOME integration.
- **Weaknesses**: Explicitly stylus-first; Windows integration is not a maintainer priority. Licensed under GPL-3.0-or-later. Suffers from unstable `.rnote` file format changes breaking across updates (Issues #1173, #1849). Direct PDF annotation conflicts with infinite-canvas vector primitives.
- **Ruqa's Differentiation**: Mouse-first ergonomics, clean-room Apache-2.0 codebase, stable forward-compatible JSON document format, and Windows/Linux parity.

### Lorien
- **Strengths**: Extremely fast infinite canvas written in Godot (GDScript). Tiny save files (15–50 KB), distraction-free Zen mode, instant pan/zoom.
- **Weaknesses**: Lacks a text tool entirely (Issue #40), lacks box connectors, uses a custom binary file format header (Issue #314), and relies heavily on tablet pressure.
- **Ruqa's Differentiation**: Full text engine with bilingual Arabic/Latin typography, geometric connectors, and human-readable versioned JSON documents.

### tldraw
- **Strengths**: Sleek UI, best-in-class arrow snapping and freehand curves.
- **Weaknesses**: Restrictive, non-OSI "tldraw SDK License" with commercial restrictions, mandatory watermarks on free tiers, and no standalone local-first desktop study application.
- **Ruqa's Differentiation**: Fully open source under Apache-2.0, no commercial telemetry, and completely offline.

### Obsidian Canvas & JSON Canvas
- **Strengths**: Open JSON Canvas 1.0 specification (`.canvas`), deep integration with local Markdown vaults.
- **Weaknesses**: Focused strictly on text cards and nodes; no freehand pen or hand-drawn rough sketch aesthetics. The desktop application itself is closed-source.
- **Ruqa's Differentiation**: Combines the fluid spatial sketching of Excalidraw with local file-based vault organization, offering JSON Canvas export in v0.2.

---

## 3. Naming Context: Lawh.ai Collision

The project was originally bootstrapped under the working title "Lawh". Market research in November 2026 confirmed that **Lawh.ai** (https://lawh.ai/features) is an active commercial software company operating a browser-based graphic design canvas with a primary marketing focus on Arabic and right-to-left typography.

To eliminate trademark infringement liability (International Classes 9 & 42), SEO confusion, and user ambiguity, the project was renamed to **Ruqa** (Arabic: Ruq'ah, meaning drawing tool or studio). See [docs/adr/0005-project-name.md](../adr/0005-project-name.md) for full audit evidence.

---

## 4. Honest Gaps: Where Ruqa Deliberately Does Not Compete

We explicitly choose **not** to compete in these areas:
1. **Real-Time Multiplayer Collaboration**: We will not build WebSocket rooms or server-managed CRDTs. Ruqa is for personal study and asynchronous export.
2. **Stylus Pressure Calibration & Wacom Drivers**: We optimize specifically for standard mouse and keyboard. We do not support 8,192 levels of pressure or tilt.
3. **Full Multi-Page PDF Editing / Form Filling**: Ruqa is an infinite diagramming canvas, not an Adobe Acrobat or Xournal++ replacement.
4. **Mobile Apps (iOS / Android)**: Focus is exclusively on desktop laptops and workstations (Linux and Windows).
5. **In-App Cloud AI Assistant**: We do not embed cloud LLM chat sidebars. External agents interact via files and open formats (`SKILL.md`).
