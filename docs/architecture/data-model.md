# Data Model Architecture

> **Status: Design Proposal (Draft)**  
> This document details the in-memory scene graph, element types, and geometry data structures inside `lawh-core`.

---

## 1. Geometric Foundations

All geometric calculations use standard 64-bit or 32-bit floating point coordinates in a continuous Cartesian plane:

```rust
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub pan: Point,
    pub zoom: f32, // Clamped between 0.1 and 10.0
}
```

### Coordinate Systems
1. **Screen Coordinates**: Pixel offsets relative to the application window $(0, 0)$ at top-left.
2. **World Coordinates**: Infinite canvas coordinates invariant to panning and zoom.
3. **Conversion Formula**:
   $$\text{world\_x} = \frac{\text{screen\_x} - \text{pan\_x}}{\text{zoom}}$$
   $$\text{world\_y} = \frac{\text{screen\_y} - \text{pan\_y}}{\text{zoom}}$$

---

## 2. Canvas Element Model

Every shape on the canvas implements a common identification and visual property model:

```rust
pub type ElementId = u64;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ElementStyle {
    pub stroke_color: String,   // Hex string (e.g., "#E0E0E0")
    pub fill_color: Option<String>,
    pub stroke_width: f32,      // Standard: 1.5 (thin), 3.0 (medium), 5.0 (thick)
    pub roughness: f32,         // Rough sketch jitter factor (default: 1.2)
    pub seed: u32,              // Deterministic seed for reproducible rough lines
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Element {
    Stroke {
        id: ElementId,
        points: Vec<Point>,
        pressures: Vec<f32>,
        style: ElementStyle,
    },
    Rectangle {
        id: ElementId,
        bounds: Rect,
        style: ElementStyle,
    },
    Ellipse {
        id: ElementId,
        bounds: Rect,
        style: ElementStyle,
    },
    Line {
        id: ElementId,
        start: Point,
        end: Point,
        style: ElementStyle,
    },
    Arrow {
        id: ElementId,
        start: Point,
        end: Point,
        style: ElementStyle,
    },
}
```

### Why Deterministic Seeds Matter
The Rough algorithm creates hand-drawn jitter using a pseudo-random generator. If shapes picked a random seed on every frame, lines would vibrate continuously ("dancing lines"). By saving a deterministic `seed: u32` per element, the jitter remains identical across frames and restarts.

---

## 3. Scene Graph & Selection State

```rust
#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub selected_ids: std::collections::HashSet<ElementId>,
}
```

---

## 4. Undo / Redo Command History

Mutations are modeled as reversible commands to maintain a lightweight history stack:

```rust
pub enum SceneCommand {
    InsertElement(Element),
    RemoveElement(ElementId),
    TranslateElements {
        ids: Vec<ElementId>,
        delta: Point,
    },
}

pub struct History {
    pub undo_stack: Vec<SceneCommand>,
    pub redo_stack: Vec<SceneCommand>,
    pub max_capacity: usize, // Default: 100
}
```
