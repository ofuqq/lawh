# Ruqa MVP Specification (v0.1.0)

## 1. Goal & Product Definition
Deliver a standalone desktop whiteboard and study scratchpad in native Rust that boots in under 150 ms, idles under 100 MB of RAM, and lets students and developers sketch diagrams, connect concepts, and write mixed Arabic/English notes using a standard mouse and keyboard on local files.

---

## 2. Target Personas
- **Primary**: Bilingual STEM Student on a Laptop & Self-Taught Developer / PKM Builder.
- **Secondary**: Live Technical Explainer / Content Creator.
- **Explicitly Excluded**: Stylus Lecture PDF Annotators (see [users-and-use-cases.md](users-and-use-cases.md)).

---

## 3. Scope Boundaries: v0.1 vs v0.2 vs v0.3+

```mermaid
flowchart LR
    subgraph v01["v0.1.0 Core MVP (M0–M5)"]
        Canvas["Infinite Dark Canvas (Pan/Zoom)"]
        Draw["Pen, Eraser & Rough Shapes (R,O,L,A)"]
        Edit["Select, Move, Resize, Delete, Undo/Redo"]
        Vault["Folder Sidebar & Atomic JSON (.ruqa)"]
        Text["Parley Text & Bundled OFL Arabic Fonts"]
        Export["PNG Image Export"]
    end

    subgraph v02["v0.2.0 Smart Features (M6)"]
        Connect["Box-to-Box Raycast Connectors"]
        AutoGrow["Bound Text Auto-Grow"]
        Search["Ctrl+P Cross-Board Search"]
        Snap["Snapping & Alignment Guides"]
        Frames["Numbered Lecture Frames"]
        Zen["Zen Mode (Alt+Z) & Image Paste"]
        JCanvas["JSON Canvas (.canvas) Export"]
    end

    subgraph v03["v0.3.0+ Advanced Study"]
        Tape["Study Tape (Occlusion Boxes)"]
        Math["Inline Math (Typst / LaTeX)"]
        RTLUI["Full Arabic UI Localization"]
        Skill["Agent SKILL.md + File Watcher"]
    end

    v01 --> v02 --> v03
```

---

## 4. Detailed Feature Breakdown

### v0.1.0 (Public Launch Scope: Milestones M0–M5)
Every item below has concrete acceptance criteria:

1. **Infinite Dark Canvas**:
   - Panning via middle-click drag or `Space + Left-click drag`.
   - Cursor-centered zooming via mouse scroll wheel (clamped 0.1x to 10.0x).
   - High-contrast dark theme by default (`#121212` background).
2. **Velocity-Simulated Freehand Pen & Segment Eraser**:
   - Freehand pen with velocity-based stroke tapering (using `freedraw` algorithm) to eliminate mouse staircase artifacts.
   - Eraser tool (`E`) that removes intersected strokes or elements on click/drag.
3. **Hand-Drawn Geometric Shapes**:
   - Rectangle (`R`), Ellipse (`O`), Line (`L`), and Arrow (`A`) tools with single-key shortcuts.
   - Procedural hand-drawn jitter generated via `roughr` core and rendered to `tiny-skia` paths.
   - Deterministic `seed: u64` saved per element to prevent stroke jitter between frames.
4. **Direct Selection & Transformation**:
   - Select tool (`V`) supporting single-element click selection and box drag multi-selection.
   - Bounding box with 8-point resize handles and drag translation.
   - Deletion via `Delete` or `Backspace`.
5. **Command-Pattern Undo / Redo**:
   - `Ctrl+Z` (Undo) and `Ctrl+Y` / `Ctrl+Shift+Z` (Redo) supporting up to 100 consecutive operations.
6. **Versioned Document Storage & Study Vault Sidebar**:
   - Single-board documents stored as human-readable JSON (`.ruqa` or `.ruqa.json`) with root `"schema_version": 1` and `"format": "ruqa"`.
   - Atomic saving (writes to `.tmp` file, then renames) to prevent file corruption on system crashes.
   - Collapsible left sidebar (`Ctrl+B`) showing boards within an open local OS folder ("Study Vault").
7. **Bilingual Text & Typography (Parley Engine)**:
   - Text rendering on canvas powered by Linebender's `parley` (using `harfrust` HarfBuzz port and `icu4x`).
   - Bundled OFL-licensed fonts: **Aref Ruqaa** (display/headings) and **Vazirmatn** (body notes).
   - Automatic paragraph direction detection with strict LTR isolation for embedded English code or math tokens.
   - Basic on-canvas text editing (double-click to activate text input buffer).
8. **PNG Export**:
   - `Ctrl+E` exports a cropped 32-bit RGBA PNG image of canvas elements with dark or transparent background.

---

### v0.2.0 (Milestone M6: Smart Study & Interoperability)
- **Bound Text with Auto-Grow**: Typing inside a rectangle or ellipse automatically expands the container's vertical bounds when text wraps.
- **Smart Box-to-Box Raycast Connectors**: Arrows pinned to element boundaries (using a ray-AABB intersection algorithm) that stay connected when shapes move.
- **Snapping Guides**: Visual alignment lines showing center and edge alignment with nearby shapes during drag.
- **Cross-Board Search (`Ctrl+P`)**: Instant fuzzy search across filenames, frame titles, and text contents within the current study vault.
- **Numbered Lecture Frames**: 16:9 and A4 frame containers serving as presentation slides (`PageDown` / `PageUp`) and table-of-contents bookmarks.
- **Zen Mode (`Alt+Z`)**: Hides all sidebar and toolbar chrome for distraction-free sketching.
- **Clipboard Image Paste (`Ctrl+V`)**: Pasting PNG/JPEG screenshots directly onto the canvas.
- **JSON Canvas (`.canvas`) Export**: One-click export enabling boards to be opened directly in Obsidian Canvas.

---

### v0.3.0+ (Advanced Study & Ecosystem Expansion)
- **Study Tape (Active-Recall Occlusion Boxes)**: Colored tape strips that mask underlying terms or definitions, revealable on click for active recall flashcard review.
- **Inline Math Rendering**: Rendering mathematical formulas via a Typst or LaTeX subset.
- **Full Arabic UI Localization**: Right-to-left mirrored interface layout with Arabic translations.
- **External AI Agent Integration**: Standardized `SKILL.md` specification and file-watcher hot-reloading so external agents (Claude Code, Cursor, Zed) can generate and edit `.ruqa` boards on disk.

---

## 5. Explicit WON'T List (Non-Goals with Rationale)

1. ❌ **Real-Time Multiplayer Collaboration**:
   - *Rationale*: Multi-user CRDT synchronization (e.g., Loro/Automerge over WebSockets) introduces enormous architectural complexity and server operational costs. Ruqa is strictly designed for solo study and asynchronous sharing.
2. ❌ **Native Stylus / Tablet Pressure Calibration**:
   - *Rationale*: Eliminates driver maintenance sinkholes (Windows Ink vs Wintab, winit tablet regressions) and keeps development focused on the underserved mouse-first demographic.
3. ❌ **Full Multi-Page PDF Editing / Redaction**:
   - *Rationale*: Rnote maintainers confirmed (Issues #1173, #1455) that full in-place PDF multi-page editing conflicts fundamentally with infinite-canvas vector data models.
4. ❌ **Built-In Cloud LLM / AI Chat Client**:
   - *Rationale*: Embedding cloud AI wrappers adds subscription management, API keys, and bloat to a local-first tool. External AI agents interact via local files and open formats (`SKILL.md`).
5. ❌ **Mobile Builds (iOS / Android)**:
   - *Rationale*: Preserves engineering focus on desktop Windows and Linux ergonomics.
6. ❌ **Cloud Sync / Proprietary Accounts**:
   - *Rationale*: Users manage their own sync via Git, Syncthing, or standard cloud folders (Google Drive, Dropbox, OneDrive).

---

## 6. Success Metrics at 90 Days Post-Launch

- **Cold Boot Time**: $< 150\text{ ms}$ on standard modern hardware.
- **Memory Footprint**: $< 100\text{ MB}$ resident memory (RSS) with a 1,000-element canvas.
- **Rendering Throughput**: Sustained 60 FPS ($< 16.6\text{ ms}$ frame time) during continuous panning and zooming.
- **Adoption & Community (Hypotheses)**: 100+ GitHub stars, 200+ binary downloads, and at least 15 real study boards created by the owner during daily dogfooding.

---

## 7. Open-Source Launch Requirements (v0.1.0 Checklist)

Before public announcement:
- [ ] Root `README.md` with an 8-second 60 FPS demo video/GIF illustrating single-key shapes, mouse pen, and Arabic typography.
- [ ] Apache-2.0 License, `NOTICE`, and `CONTRIBUTING.md` guide.
- [ ] Zero-install prebuilt binaries for Linux (`.tar.gz` / `.AppImage`) and Windows (`.zip`) published on GitHub Releases.
- [ ] Bundled `examples/study-vault/` containing 3 realistic boards (Algorithms diagram, bilingual system architecture, circuit analysis).
- [ ] Documented JSON schema specification in `docs/architecture/file-format.md`.
