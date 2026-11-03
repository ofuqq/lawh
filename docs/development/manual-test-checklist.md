# Ruqa v0.1.0 Release Manual Test Checklist

> **Target Release**: v0.1.0 MVP  
> **Applicable Platforms**: Linux (X11 / Wayland), macOS 12+ (Intel / Apple Silicon)  
> **Prerequisites**: Native desktop binary built with `cargo build --release --package ruqa-app`

This document defines the formal, human-executed manual quality verification checklist required before finalizing any release candidate tag. Every test procedure includes explicit setup conditions, step-by-step physical input interactions, and unambiguous **Pass Criteria**.

---

## Table of Contents
1. [Canvas Navigation & HiDPI Zoom Anchoring](#1-canvas-navigation--hidpi-zoom-anchoring)
2. [Freehand Pen Velocity & Smoothing](#2-freehand-pen-velocity--smoothing)
3. [Shape Tools & Hand-Drawn Rendering](#3-shape-tools--hand-drawn-rendering)
4. [Selection, Translation & Transform Handles](#4-selection-translation--transform-handles)
5. [Continuous Eraser Tool](#5-continuous-eraser-tool)
6. [Styling Panel & Theme Inversion](#6-styling-panel--theme-inversion)
7. [Document Persistence, Autosave & Quit Guard](#7-document-persistence-autosave--quit-guard)
8. [High-Resolution PNG Export](#8-high-resolution-png-export)
9. [Recent Files HUD & Study Vault Sidebar](#9-recent-files-hud--study-vault-sidebar)
10. [Multi-Monitor & DPI Scaling Transitions](#10-multi-monitor--dpi-scaling-transitions)

---

## 1. Canvas Navigation & HiDPI Zoom Anchoring

### Test 1.1: Middle-Click and Space+Drag Panning
- **Objective**: Verify infinite canvas panning is buttery-smooth with zero cursor drift.
- **Steps**:
  1. Launch Ruqa on a display with default 1.0x or 2.0x scaling.
  2. Draw a rectangle in the center of the window.
  3. Press and hold middle mouse button (or press `Space` and hold left mouse button).
  4. Drag cursor 300 pixels to the left, then release.
  5. Repeat dragging in circles and rapid back-and-forth gestures.
- **Pass Criteria**:
  - The rectangle tracks 1:1 with cursor displacement in real-time.
  - Release terminates panning instantly without inertia glitch or hitching.
  - Cursor switches to grabbing hand (`ClosedHand`) during drag and reverts upon release.

### Test 1.2: Anchored Mouse Wheel Zoom on HiDPI Displays
- **Objective**: Confirm zoom focuses mathematically on the world point underneath cursor on 100%, 150%, and 200% DPI displays.
- **Steps**:
  1. Position cursor directly over the top-left vertex of an existing rectangle.
  2. Rapidly scroll mouse wheel down 5 detents (zoom in).
  3. Observe position of the rectangle's vertex relative to the cursor.
  4. Rapidly scroll mouse wheel up 5 detents (zoom out).
- **Pass Criteria**:
  - The vertex under the cursor remains stationary under the pointer during both zoom-in and zoom-out operations.
  - Zoom level clamps smoothly between 0.1x (minimum) and 5.0x (maximum) without crash or NaN transform matrix.
  - No visual stutter or pixel snapping jitter on HiDPI displays (scale factor 1.25x, 1.5x, 2.0x).

---

## 2. Freehand Pen Velocity & Smoothing

### Test 2.1: Slow vs Rapid Mouse Flick Dynamics
- **Objective**: Verify velocity-modulated pressure simulation and Catmull-Rom spline smoothing.
- **Steps**:
  1. Press `P` to select the Freehand Pen tool.
  2. Draw a slow, deliberate cursive script word (e.g. "Ruqa") taking ~4 seconds.
  3. Adjacent to it, perform 3 extremely fast flick strokes across the screen (~100ms per stroke).
- **Pass Criteria**:
  - Slow strokes produce consistent, thick, ink-like contours without angular polygon staircasing.
  - Fast flicks taper off smoothly to thin tips according to velocity modulation without polygon self-intersection glitches or artifacts.
  - Both strokes rasterize cleanly with zero frame drops (< 16.6ms per frame).

---

## 3. Shape Tools & Hand-Drawn Rendering

### Test 3.1: Geometric Shape Primitives Creation
- **Objective**: Verify drawing rectangles, ellipses, diamonds, lines, and arrows.
- **Steps**:
  1. Press `R`, click-drag to place a rectangle.
  2. Press `O`, click-drag to place an ellipse.
  3. Press `D`, click-drag to place a diamond.
  4. Press `L`, click-drag to place a line.
  5. Press `A`, click-drag to place an arrow.
- **Pass Criteria**:
  - Each shape displays characteristic rough hand-drawn two-pass strokes.
  - Ellipses maintain curvature; diamonds align to 4 bounding box midpoints; arrows display clear triangular head points at the terminal point.
  - Shapes retain their exact rough appearance upon pan and zoom without re-randomizing (stable PRNG seed).

---

## 4. Selection, Translation & Transform Handles

### Test 4.1: Click & Marquee Selection
- **Objective**: Verify single-element and multi-element selection logic.
- **Steps**:
  1. Press `V` to select the Selection tool.
  2. Click on a drawn shape outline or filled interior.
  3. Click on empty canvas.
  4. Click-drag a rectangular marquee over 3 shapes.
- **Pass Criteria**:
  - Clicking a shape highlights it with a blue bounding outline and 8 resize handles.
  - Clicking empty canvas deselects all items.
  - Marquee box selects all elements intersecting or enclosed by the marquee.

### Test 4.2: Translation and Corner Handle Resizing
- **Objective**: Verify translating and proportional/axis resizing.
- **Steps**:
  1. Select a shape and drag its body across the canvas.
  2. Drag the bottom-right corner handle diagonally outward.
  3. Drag the right-edge midpoint handle horizontally inward.
  4. Drag a corner handle past the opposite boundary to flip coordinates.
- **Pass Criteria**:
  - Dragging moves element smoothly with immediate visual feedback.
  - Corner handle scales both width and height.
  - Edge handle scales only the horizontal axis, keeping vertical dimension constant.
  - Minimum element dimension clamps gracefully at 10 world units (no negative size crash).

### Test 4.3: Keyboard Manipulation (Duplicate, Delete, Undo/Redo)
- **Objective**: Verify keyboard commands for direct manipulation.
- **Steps**:
  1. Select an element, press `Ctrl+D`.
  2. With new element selected, press `Delete`.
  3. Press `Ctrl+Z` twice.
  4. Press `Ctrl+Y` once.
- **Pass Criteria**:
  - `Ctrl+D` clones element offset by +20px horizontally and vertically.
  - `Delete` removes element from canvas and clears selection.
  - `Ctrl+Z` restores deleted element and reverts duplication.
  - `Ctrl+Y` re-executes undone actions without state corruption.

---

## 5. Continuous Eraser Tool

### Test 5.1: Drag Erasing Multiple Elements
- **Objective**: Confirm continuous sweep eraser across vector shapes and ink paths.
- **Steps**:
  1. Draw 5 overlapping shapes and several freehand pen strokes.
  2. Press `E` to activate Eraser tool.
  3. Click-drag in a continuous stroke cutting through all 5 shapes and strokes.
  4. Release mouse button, then press `Ctrl+Z`.
- **Pass Criteria**:
  - Elements vanish immediately as the eraser cursor passes over their boundaries.
  - All erased elements in that continuous drag are grouped into a single atomic undo transaction; `Ctrl+Z` restores all 5 shapes simultaneously.

---

## 6. Styling Panel & Theme Inversion

### Test 6.1: Color Palette, Stroke Width & Fill Patterns
- **Objective**: Verify styling panel adjustments on active and selected elements.
- **Steps**:
  1. Select an uncolored rectangle.
  2. Click Coral (`#FF6B6B`) color chip in styling panel.
  3. Click Bold (`4.0px`) stroke width button.
  4. Click `Hachure` fill pattern.
- **Pass Criteria**:
  - The rectangle immediately renders with thick coral borders and sketchy parallel hachure lines.
  - Seed reproducibility ensures hatch lines remain static across frame redraws.

### Test 6.2: Dark/Light Mode Inversion
- **Objective**: Test contrast preservation during theme toggle (`Ctrl+T` or `Ctrl+I`).
- **Steps**:
  1. Draw shapes using white, black, and coral strokes.
  2. Press `Ctrl+T` (or `Ctrl+I`) to toggle to Light Theme.
  3. Press `Ctrl+T` (or `Ctrl+I`) again to return to Dark Theme.
- **Pass Criteria**:
  - In light theme, background transitions to crisp light grey/white (`#F8F9FA`), monochrome strokes invert to dark ink (`#1E1E1E`), while coral accent remains clearly legible.
  - Canvas dot grid color adjusts brightness automatically to maintain low-contrast visibility.

---

## 7. Document Persistence, Autosave & Quit Guard

### Test 7.1: Save, Open & Atomic Autosave
- **Objective**: Validate `.ruqa` JSON serialization and crash-safe autosave.
- **Steps**:
  1. Draw several shapes on a new board.
  2. Observe window title shows asterisk `*` indicating dirty state.
  3. Wait 30 seconds without mouse movement. Inspect file system for temp autosave file.
  4. Press `Ctrl+S`, enter `test_board.ruqa` in file dialog, and save.
- **Pass Criteria**:
  - Saving removes dirty asterisk from window title and updates path display.
  - File written to disk is valid JSON matching `RuqaDocument` schema version 1.
  - Reopening `test_board.ruqa` reproduces elements, coordinates, and styling with zero loss.

### Test 7.2: Unsaved Changes Quit Dialog
- **Objective**: Verify data loss protection upon quit.
- **Steps**:
  1. Modify a saved board so window title shows `*`.
  2. Click window close button `X` (or press `Ctrl+Q`).
  3. In the modal dialog, click `Cancel`.
  4. Press `Ctrl+Q` again, click `Don't Save`.
- **Pass Criteria**:
  - When dirty, close request intercepts exit and renders modal with `Save`, `Don't Save`, and `Cancel` buttons.
  - `Cancel` dismisses dialog and leaves canvas completely interactive.
  - `Don't Save` discards unwritten edits and closes application cleanly.

---

## 8. High-Resolution PNG Export

### Test 8.1: Full Canvas vs Tight Selection Export
- **Objective**: Confirm PNG export at 2x HiDPI resolution.
- **Steps**:
  1. Create a diagram with 3 shapes grouped on the left and 1 shape far to the right.
  2. Deselect all elements. Press `Ctrl+E` and save to `full.png`.
  3. Select only the 3 shapes on the left. Press `Ctrl+E` and save to `selection.png`.
  4. Open both PNG files in an image viewer.
- **Pass Criteria**:
  - `full.png` contains all 4 shapes with uniform 20px padding around their aggregate bounds.
  - `selection.png` contains strictly the 3 selected shapes cropped tightly.
  - Images are crisp 2x resolution with clean raster lines and dark background matching canvas theme.

---

## 9. Recent Files HUD & Study Vault Sidebar

### Test 9.1: Startup Recent Files HUD
- **Objective**: Verify fast reopening of previous sessions.
- **Steps**:
  1. Launch Ruqa without arguments.
  2. Verify recent files HUD displays list of recently opened `.ruqa` files.
  3. Click a file item in the HUD.
- **Pass Criteria**:
  - The clicked board loads into the viewport immediately.
  - HUD dismisses once a document is active.

### Test 9.2: Study Vault Folder Navigation
- **Objective**: Browse local study folders and switch boards smoothly.
- **Steps**:
  1. Press `Ctrl+B` to open Study Vault sidebar.
  2. Select a folder containing subfolders and multiple `.ruqa` files.
  3. Expand subfolders with chevron clicks and click a board file.
- **Pass Criteria**:
  - Directory hierarchy reflects nested structure.
  - Canvas clicks are isolated when clicking inside sidebar drawer bounds.
  - Switching boards prompts confirmation if current board has unsaved edits.

---

## 10. Multi-Monitor & DPI Scaling Transitions

### Test 10.1: Window Dragging Across Mixed-DPI Displays
- **Objective**: Test dynamic HiDPI scale factor transitions without blurry rendering or coordinate scaling errors.
- **Steps**:
  1. On a multi-monitor setup (e.g. 1.0x 1080p primary + 2.0x 4K secondary display):
  2. Open Ruqa on the 1.0x display. Draw shapes and verify sharp rendering.
  3. Drag the window across to the 2.0x display.
  4. Draw new shapes and test mouse wheel zooming.
- **Pass Criteria**:
  - Softbuffer pixmap surface reallocates to physical pixel dimensions immediately upon receiving `ScaleFactorChanged`.
  - Lines and shapes render at native 2.0x Retina sharpness without blurriness or clipping.
  - Mouse coordinate hit-testing remains 100% accurate without 2x offset displacement.
