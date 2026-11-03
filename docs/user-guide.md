# Ruqa User Guide

> **Version**: 0.1.0 · **Platform**: Linux, macOS (desktop native app)

Ruqa is a lightweight, open-source desktop whiteboard with a hand-drawn aesthetic.
It is designed for **mouse-first use** — studying, explaining concepts, and sketching diagrams.

---

## Table of Contents

1. [Quickstart](#1-quickstart)
2. [Mouse Controls](#2-mouse-controls)
3. [Drawing Tools](#3-drawing-tools)
4. [Selection and Editing](#4-selection-and-editing)
5. [Keyboard Shortcut Reference](#5-keyboard-shortcut-reference)
6. [File Management](#6-file-management)
7. [PNG Export](#7-png-export)
8. [Dark and Light Theme](#8-dark-and-light-theme)
9. [Study Vault](#9-study-vault)
10. [Tips for Students](#10-tips-for-students)

---

## 1. Quickstart

### Launch Ruqa

```bash
# From the project directory (development build):
cargo run --release --package ruqa-app
```

When Ruqa opens you see a dark canvas with a floating toolbar at the top.
You are ready to draw immediately — no account, no cloud, no browser required.

### Save your first board

1. Draw something.
2. Press **Ctrl+S** (or **⌘S** on macOS). A native file dialog opens.
3. Choose a location and filename. Ruqa saves a `.ruqa` file (plain JSON).

### Autosave

Ruqa autosaves your open document every 30 seconds to a crash-safe temp file.
If the app closes unexpectedly, reopen the original file path; Ruqa will detect
and offer to restore the autosave.

---

## 2. Mouse Controls

Ruqa is designed to feel natural with a trackpad or a standard three-button mouse.

| Action | Mouse gesture |
|:---|:---|
| **Pan canvas** | Middle-click and drag · or · Space + left-click drag |
| **Zoom in/out** | Scroll wheel (zooms toward cursor position) |
| **Draw / Place shape** | Left-click or left-click drag (depends on active tool) |
| **Select element** | Left-click on element (Select tool active) |
| **Marquee select** | Left-click drag on empty canvas (Select tool active) |
| **Move selected** | Left-click drag on selected element |
| **Resize selected** | Drag any of the 8 corner/edge resize handles |
| **Context menu** | Right-click (future feature — not yet in v0.1) |

> **Tip — trackpad users**: Two-finger scroll pans/zooms the canvas. Pinch-to-zoom is not yet supported; use the scroll wheel or the keyboard zoom shortcuts instead.

---

## 3. Drawing Tools

Activate a tool by clicking its icon in the top toolbar **or** pressing its keyboard shortcut.

| Icon | Tool | Shortcut | What it does |
|:---:|:---|:---:|:---|
| ↖ | **Select** | `V` | Click or marquee-drag to select; drag handles to move/resize |
| ✏ | **Pen** | `P` | Freehand ink stroke with velocity-based pressure simulation |
| □ | **Rectangle** | `R` | Drag to draw a hand-drawn rough rectangle |
| ◇ | **Diamond** | `D` | Drag to draw a rough diamond (rhombus) |
| ○ | **Ellipse** | `O` | Drag to draw a rough ellipse or circle |
| — | **Line** | `L` | Drag to draw a straight rough line segment |
| → | **Arrow** | `A` | Drag to draw a directional arrow |
| ⌫ | **Eraser** | `E` | Click or drag continuously to erase elements under the cursor |

### Styling a shape

After drawing a shape (or while an element is selected), the **Styling Panel** appears on the right edge of the canvas. Use it to change:

- **Stroke color** — choose from 6 curated palette colors or click More for any hex color.
- **Stroke width** — Thin (1 px) · Medium (2.5 px) · Bold (4 px).
- **Fill style** — None · Solid · Semi-transparent · Hachure · Cross-hatch.
- **Roughness** — Clean (straight) ↔ Sketchy (hand-drawn jitter).

### Pen tool tips

- Draw **slowly** for smoother, more precise strokes.
- Draw **quickly** for lighter, thinner strokes (velocity-based pressure modulation).
- Ruqa smooths strokes automatically using Catmull-Rom interpolation.

---

## 4. Selection and Editing

### Selecting elements

- **Single click** on an element (Select tool) → selects it.
- **Click on empty canvas** → deselects all.
- **Drag a marquee** on empty canvas → selects all elements inside the box.
- **Shift+click** is not yet supported in v0.1; use marquee for multi-select.

### Moving and resizing

- Drag a selected element anywhere on the canvas to move it.
- Drag one of the 8 resize handles (corners + edge midpoints) to resize.
  - Corner handles scale both dimensions.
  - Edge midpoint handles scale a single axis only.

### Copy, paste, duplicate

| Action | Shortcut |
|:---|:---|
| Copy | Ctrl+C |
| Paste | Ctrl+V |
| Duplicate (copy+paste in one step) | Ctrl+D |

Duplicated elements are placed 20 world-units to the right and below the original.

### Delete

Press `Delete` or `Backspace` to remove all currently selected elements.

### Undo / Redo

| Action | Shortcut |
|:---|:---|
| Undo | Ctrl+Z |
| Redo | Ctrl+Y · or · Ctrl+Shift+Z |

Ruqa maintains unlimited undo history for the current session.
The history is **not** persisted to the `.ruqa` file (history is cleared on file open).

---

## 5. Keyboard Shortcut Reference

### Tools

| Key | Tool |
|:---:|:---|
| `V` | Select |
| `P` | Pen (freehand) |
| `R` | Rectangle |
| `D` | Diamond |
| `O` | Ellipse |
| `L` | Line |
| `A` | Arrow |
| `E` | Eraser |

### Canvas navigation

| Shortcut | Action |
|:---|:---|
| Scroll wheel | Zoom in / out toward cursor |
| Space + drag | Pan canvas |
| Middle-click drag | Pan canvas |

### Editing

| Shortcut | Action |
|:---|:---|
| Ctrl+Z | Undo |
| Ctrl+Y / Ctrl+Shift+Z | Redo |
| Ctrl+C | Copy selected elements |
| Ctrl+V | Paste |
| Ctrl+D | Duplicate selected elements |
| Delete / Backspace | Delete selected elements |

### File

| Shortcut | Action |
|:---|:---|
| Ctrl+S | Save (opens file dialog on first save) |
| Ctrl+O | Open file (or open Recent Files HUD) |
| Ctrl+N | New board |
| Ctrl+B | Toggle Study Vault sidebar |
| Ctrl+E | Export canvas as PNG |
| Ctrl+Q / Ctrl+W | Quit (shows confirmation dialog if unsaved) |

### View

| Shortcut | Action |
|:---|:---|
| Ctrl+` (backtick) | Toggle canvas dot grid |
| Ctrl+T / Ctrl+I | Toggle dark/light theme |

---

## 6. File Management

### Open a board

- Press **Ctrl+O** to open the file dialog and choose a `.ruqa` file.
- The **Recent Files HUD** (top of canvas on startup, also Ctrl+O) lists the last 10
  boards you opened — click any entry to reopen it.

### New board

Press **Ctrl+N**. If the current board has unsaved changes, the **Quit Dialog** (Save /
Don't Save / Cancel) appears first to prevent accidental data loss.

### Save and Save As

- **Ctrl+S**: Saves to the current file path. Opens a file dialog if the board has never
  been saved.
- No separate "Save As" shortcut in v0.1 — close and reopen, then save to a new path.

### .ruqa file format

`.ruqa` files are **plain UTF-8 JSON** you can inspect or edit with any text editor.
The format is versioned (`schema_version: 1`) and forward-compatible.

```jsonc
{
  "format": "ruqa",
  "schema_version": 1,
  "app_version": "0.1.0",
  "canvas": { "background_color": "#121212", "viewport": { ... } },
  "elements": [ ... ]
}
```

See [`docs/architecture/file-format.md`](../architecture/file-format.md) for the full specification.

---

## 7. PNG Export

Export a high-resolution PNG snapshot of your board:

1. Press **Ctrl+E** (or choose _File → Export PNG_ from the menu).
2. A file dialog opens. Choose a destination `.png` file.
3. Ruqa renders the board at **2× pixel density** (HiDPI quality) and writes the file.

### Tight vs full-canvas export

- If **elements are selected**, Ruqa exports only the bounding box of those elements
  (tight crop, with 20 px padding). Useful for diagram screenshots.
- If **nothing is selected**, Ruqa exports the entire scene bounding box.

The viewport pan/zoom does not affect the export; the full element bounds are always captured.

---

## 8. Dark and Light Theme

Ruqa defaults to a dark canvas (`#121212` background with off-white strokes).
Press **Ctrl+T** (or **Ctrl+I**) to toggle the light theme (white background with dark strokes).

The theme toggle inverts stroke and background colors while preserving accent colors
(coral, amber, green, cyan, lavender) so diagrams remain readable in both modes.

---

## 9. Study Vault

The **Study Vault** is a local folder-browser sidebar for organizing your `.ruqa` boards.

### Open the sidebar

Press **Ctrl+B** or click the Vault icon in the toolbar.

### Set a vault folder

The first time you open the sidebar, Ruqa asks you to choose a root folder.
All `.ruqa` files inside that folder tree are listed in the sidebar.

### Open a board from the vault

Click any file in the sidebar list to open it instantly.
The current board is saved (with confirmation if unsaved) before switching.

### Tips

- Keep related boards in named subfolders: `algorithms/`, `networking/`, `lecture-notes/`.
- The sidebar shows only `.ruqa` files; other file types in the folder are hidden.

---

## 10. Tips for Students

| Goal | Recommended workflow |
|:---|:---|
| **Explain a concept step by step** | Use arrows to connect rough shapes; use the Pen tool for annotations |
| **Compare two approaches** | Draw two groups of shapes on opposite halves of the canvas |
| **Save a snapshot mid-session** | Press Ctrl+E to export PNG; keep drawing without losing the "in-progress" state |
| **Organize lecture notes** | One `.ruqa` file per lecture; store all in a Study Vault folder |
| **Present to someone else** | Zoom out (scroll wheel) so the whole diagram fits the screen |
| **Quick diagram for a study card** | Export tight selection PNG (select shapes, Ctrl+E) |

---

## System Requirements

| Component | Requirement |
|:---|:---|
| OS | Linux (X11 or Wayland) · macOS 12+ |
| CPU | Any modern x86-64 or ARM64 |
| RAM | < 100 MB peak (pure CPU rasterization, no GPU required) |
| Display | Any resolution; HiDPI/Retina displays are fully supported |
| Dependencies (Linux) | `libxkbcommon`, `libxkbcommon-x11` (for keyboard input under X11) |

---

*For architecture and file format details see [`docs/architecture/`](../architecture/).*
*For contribution guidelines see [`CONTRIBUTING.md`](../../CONTRIBUTING.md).*

