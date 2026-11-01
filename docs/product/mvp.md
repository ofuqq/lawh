# Lawh MVP Specification (v0.1.0)

## 1. Goal & Vision Statement
Deliver a rock-solid, standalone Linux desktop whiteboard that boots in under 100ms, lets a student or developer sketch diagrams and explain concepts with a smooth hand-drawn look using only a mouse, saves to a versioned local `.lawh` file, and exports high-resolution PNG snapshots.

## 2. Target User
The solo technical learner, student, or developer who wants an instant, offline, zero-account sketching scratchpad to visually decompose algorithms, systems, and study notes.

---

## 3. Scope Boundaries: MUST / SHOULD / WON'T

### MUST Have (v0.1 Hard Requirements)
Every MUST item has strict acceptance criteria.

1. **Infinite Pan & Zoom Canvas**
   - *Acceptance Criteria*:
     - Middle-click drag and `Space + Left-click` drag pan the canvas smoothly.
     - Scroll wheel zooms smoothly in and out centered on the current mouse pointer position.
     - Zoom levels constrained safely between 10% (0.1x) and 1000% (10.0x).
     - Maintains 60 FPS panning on standard 1080p desktop display.
2. **Smooth Freehand Pen Tool**
   - *Acceptance Criteria*:
     - User can draw continuous freehand lines using left-click drag.
     - Velocity-based stroke smoothing eliminates mouse staircase artifacts.
     - Color palette provides at least 5 standard high-contrast dark-mode colors (White, Slate, Cyan, Green, Coral/Red).
     - Stroke width selector (Thin, Medium, Thick).
3. **Hand-Drawn Basic Shapes**
   - *Acceptance Criteria*:
     - Rectangle, Ellipse, Line, and Arrow tools.
     - Rendered with organic hand-drawn sketchiness (two-pass jittered contour via Rough algorithm).
     - Arrowheads auto-align to line endpoint angle.
4. **Direct Selection & Manipulation**
   - *Acceptance Criteria*:
     - Selection tool allows clicking an element or dragging a bounding selection box to select multiple shapes.
     - Selected elements show an active bounding box with corner resize handles.
     - Dragging inside bounding box translates selected elements.
     - Hitting `Delete` or `Backspace` removes selected elements from the canvas.
5. **Unlimited Undo / Redo**
   - *Acceptance Criteria*:
     - `Ctrl+Z` reverses the last user action (stroke, shape creation, move, delete).
     - `Ctrl+Y` or `Ctrl+Shift+Z` redoes the action.
     - History stack stores up to 100 consecutive operations without memory growth anomalies.
6. **Persistence & Versioned File Format (`.lawh`)**
   - *Acceptance Criteria*:
     - `Ctrl+S` saves the entire canvas state to a `.lawh` JSON document.
     - File format includes a top-level `"version": 1` field and forward-compatible structure.
     - `Ctrl+O` opens a `.lawh` file from disk and reconstructs the canvas exactly.
7. **PNG Image Export**
   - *Acceptance Criteria*:
     - `Ctrl+E` exports the canvas contents to a cropped 32-bit RGBA PNG image matching element bounds.
     - User can toggle dark-theme background inclusion vs transparent PNG export.
8. **Rendering Performance Benchmark (Decision 7 Exit Criterion)**
   - *Acceptance Criteria*:
     - Automated headless benchmark tests frame rendering time with 1,000, 5,000, and 10,000 strokes.
     - Target for CPU tiny-skia renderer: **Frame render time < 16.6 ms (sustaining 60 FPS) at 1,000 strokes**.
     - Provides clear data points to evaluate whether GPU acceleration (`vello`) is required for larger boards.

### SHOULD Have (v0.1 Stretch Goals)
- [ ] Keyboard shortcut cheat sheet modal (`?` key).
- [ ] Shape duplication via `Ctrl+D` or `Alt + Drag`.
- [ ] Stroke fill styles (Hachure, Solid, Cross-hatch).
- [ ] Multi-level grid toggle (Subtle dots or plain background).

### WON'T Have in v0.1 (Explicit Non-Goals)
- ❌ **No Real-Time Collaboration**: No WebSockets, WebRTC, multiplayer cursors, or server syncing.
- ❌ **No User Accounts / Cloud**: No logins, authentication, or cloud drives.
- ❌ **No Full Arabic Text Shaping**: Arabic/RTL text rendering requires HarfBuzz BiDi shaping (scheduled for v0.2).
- ❌ **No Complex Text Layout**: Text tool in v0.1 is either omitted or strictly limited to basic ASCII labels.
- ❌ **No Mobile or Web Builds**: Native desktop (Linux) is the exclusive tier-1 target for v0.1.

---

## 4. Success Metrics

1. **Cold Boot Time**: Binary launches to an interactive canvas in `< 100 ms` on modern Linux desktops.
2. **Idle Memory**: Process uses `< 50 MB` RSS when running an empty or small canvas.
3. **Input Responsiveness**: Pen draw input lag `< 16 ms` between mouse movement and raster update.
4. **Zero-Crash Stability**: 0 panics during continuous 30-minute drawing and undo/redo stress testing.

---

## 5. Minimum Open-Source Launch Requirements

Before announcing v0.1.0 to the community, the repository must have:
1. **Compelling README**: Clear pitch, pre-alpha status banner, roadmap, and placeholder for a 5-second demo GIF.
2. **Open Source Governance**: Dual MIT/Apache-2.0 license, CONTRIBUTING guide, Contributor Covenant Code of Conduct.
3. **Automated CI**: GitHub Actions verifying `cargo fmt`, `cargo clippy`, and unit tests on every pull request.
4. **Issue & PR Templates**: Structured bug reports, feature requests, and maintainer task templates.
5. **Packaged Linux Binaries**: Automated GitHub Release delivering a standalone Linux `tar.gz` and AppImage/binary.
6. **Sample Files**: A directory of bundled `.lawh` sample diagrams (e.g., `samples/welcome.lawh`, `samples/system-architecture.lawh`).
