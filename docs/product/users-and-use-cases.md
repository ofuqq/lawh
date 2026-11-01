# Users and Use Cases

To maintain disciplined focus, Mirsam targets two primary user roles, supports one secondary role, and explicitly excludes one common non-target role.

---

## 1. Primary User Roles

### Role 1: The Bilingual STEM Student on a Laptop
- **Hardware & OS**: Mid-range Windows 11 or Linux laptop, 8–16 GB RAM, standard external mouse or trackpad, no drawing tablet.
- **Job-to-Be-Done**: Solve university computer science, electrical circuit, or algorithm exercises; draw system diagrams; and write mixed Arabic and English study notes during exam prep.
- **Pain Points with Current Tools**:
  - Web wrappers like Goodnotes on Windows consume ~1.3 GB of idle RAM and suffer 1–2s pen latency (reported by Noteastic Blog, May 2026).
  - Excalidraw lacks bundled Arabic sketchy fonts (GitHub Issue #8394) and suffers from reversed shortcut strings and inverted punctuation in RTL mode (GitHub Issues #12387, #12404).
  - OneNote freezes and syncs unpredictably on large infinite canvases.
- **Why Mirsam Wins**: Instant cold start, sub-100 MB RAM consumption, bundled Arabic hand-drawn fonts (Aref Ruqaa and Vazirmatn), and a local folder sidebar organizing boards by course.

### Role 2: The Self-Taught Developer & PKM Builder
- **Hardware & OS**: Linux (Ubuntu/Arch) or macOS laptop, mechanical keyboard, mouse or trackpad.
- **Job-to-Be-Done**: Sketch software architecture, data structures, and mental models that live inside a local Git-managed directory or personal knowledge management (PKM) vault.
- **Pain Points with Current Tools**:
  - Obsidian Canvas lacks freehand drawing and sketchy hand-drawn aesthetics.
  - Excalidraw web isolates boards inside browser local storage without a native file-tree directory.
  - Lorien lacks a text tool and shape connectors entirely.
- **Why Mirsam Wins**: Plain versioned JSON files on disk, Vim-like single-key shortcuts, fast `Ctrl+P` cross-board search, and eventual JSON Canvas interoperability in v0.2.

---

## 2. Secondary User Role

### Role 3: The Live Technical Explainer & Content Creator
- **Hardware & OS**: Desktop PC or laptop (Linux, Windows, macOS), external monitor, standard mouse.
- **Job-to-Be-Done**: Explain code, system architecture, and technical concepts live on Zoom or in OBS recordings with high contrast and zero frame drops.
- **Pain Points with Current Tools**:
  - Web tools display UI clutter (sharing modals, avatars, connection banners) during screen capture.
  - Heavy apps drop frames when screen sharing at 1440p or 4K.
- **Why Mirsam Wins**: Clean dark-theme canvas by default, minimal UI chrome that recedes while drawing, and steady 60 FPS rendering on CPU.

---

## 3. Explicit Non-Target Role

### Excluded Role: The Stylus Lecture PDF Annotator
- **Profile**: A student using an iPad Air with Apple Pencil or a Surface Pro with Surface Pen importing 80-page professor PDF slide decks to write handwritten margin notes during lectures.
- **Reason for Exclusion**:
  1. **Infinite Canvas Vector Mismatch**: As established by Rnote maintainers in Issues #1173 and #1455, full in-place PDF multi-page editing conflicts fundamentally with infinite-canvas vector rendering.
  2. **Tablet Driver Fragility**: Supporting active stylus tilt, palm rejection, and pressure sensitivity exposes a solo developer to severe platform regressions (e.g., winit's historical Windows Ink vs Wintab conflicts and startup crashes with Wacom/XP-Pen hardware in winit #4493, #4591).
  3. **High Competition**: Mature apps like Rnote, Xournal++, and Goodnotes already target stylus PDF annotation. Mirsam focuses strictly on mouse-first conceptual sketching.

---

## 4. Core User Scenarios

### Scenario A: Bilingual Algorithm Study (The Feynman Technique)
1. A student opens Mirsam from the terminal or app launcher in under 100 milliseconds.
2. In the folder sidebar, they select `Computer-Science/Algorithms/Dijkstra.mirsam`.
3. Pressing `R`, they drop a rough rectangle on the canvas and double-click to type:  
   `خوارزمية Dijkstra تعمل في O(V log V)`
4. Parley automatically detects the Arabic script, renders it with bundled Vazirmatn font, and isolates the English complexity notation $O(V \log V)$ left-to-right without reversing parentheses.
5. Pressing `A`, they drag an arrow from the explanation box to a sketched graph node.
6. The board is saved atomically to disk.

### Scenario B: Software Architecture Sketching
1. A developer opens Mirsam alongside their code editor.
2. Using single-key shortcuts (`R`, `O`, `L`), they quickly map out an event-driven message bus: Publisher, Queue, Consumer.
3. They use the velocity-smoothed pen (`P`) to circle a potential bottleneck in red.
4. Hitting `Ctrl+E`, they export a cropped 32-bit transparent PNG and drop it directly into a GitHub PR description.
