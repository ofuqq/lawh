# Product Principles

These six principles govern technical and design decisions in Ruqa.

---

### 1. Fast, Light, and Low-RAM
Ruqa must feel like grabbing a sheet of paper.
- Cold boot time under 150 milliseconds.
- Process resident memory (RSS) stays under 100 MB during active study.
- Steady 60 FPS panning and drawing on standard laptop CPUs.
- Avoid bulky runtimes, webviews, and unneeded background daemons.

### 2. Offline and Local-First
Your thoughts and study materials live on your computer.
- Ruqa is 100% offline. Zero network calls, zero accounts, zero analytics.
- Boards are stored as human-readable, versioned JSON files directly in normal filesystem folders.
- Compatible with existing backup and sync tools (Git, Syncthing, rsync).

### 3. Mouse-First Ergonomics
A graphics tablet is a luxury; a standard mouse and keyboard are standard equipment.
- Velocity-based stroke smoothing makes freehand mouse drawing smooth and natural.
- Single-key tool selection (R for Rectangle, O for Ellipse, L for Line, A for Arrow, P for Pen, E for Eraser, V for Select).
- One-handed canvas navigation via middle-click drag, Space+drag, and wheel zoom.

### 4. Hand-Drawn and Approachable
Precision tools create psychological resistance to exploratory sketching.
- Shapes feature slight, controlled sketchiness via the Rough algorithm.
- Imperfect diagrams invite rapid iteration and self-explanation.
- Avoid rigid CAD snap grids that slow down quick thinking.

### 5. Native Bilingual Typography (Arabic/RTL + Latin)
Arabic script is a first-class citizen from day one.
- Bundled open-source fonts for both Arabic and Latin text without requiring system font configuration.
- Native bidirectional layout (Unicode UAX#9) with strict left-to-right isolation for code and math tokens.
- No reversed keyboard shortcut labels or inverted punctuation.

### 6. Focused Simplicity
A focused tool that excels at personal study beats a multi-feature enterprise platform.
- Say "no" to enterprise features like multiplayer voting timers, video calls, and cloud databases.
- Every feature must directly serve the student or developer sketching and understanding technical concepts.
