# Data Model Architecture

> **Status: Design Proposal (Draft)**  
> This document details the in-memory scene graph, element types, and geometry data structures inside `ruqa-core`.

---

## 1. Geometric Foundations

All geometric coordinates are represented as continuous 32-bit floating point numbers in world coordinate space:

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

impl Rect {
    /// Returns true if two axis-aligned bounding boxes overlap.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub pan: Point,
    pub zoom: f32, // Clamped between 0.1 and 10.0
}
```

---

## 2. Element Model

Every element on the canvas has a unique identifier, geometric properties, styling attributes, and a deterministic random seed:

```rust
pub type ElementId = u64;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ElementStyle {
    pub stroke_color: String,   // Hex string (e.g. "#E0E0E0")
    pub fill_color: Option<String>,
    pub stroke_width: f32,      // Standard: 1.5, 3.0, 5.0
    pub roughness: f32,         // Default: 1.2
    pub seed: u64,              // Deterministic seed for reproducible rough lines
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
    Text {
        id: ElementId,
        position: Point,
        content: String,
        font_size: f32,
        style: ElementStyle,
    },
}
```

---

## 3. Command-Pattern History Stack

Every mutation to the canvas is expressed as an invertible command to ensure robust undo and redo without memory leaks:

```rust
pub enum Command {
    AddElement(Element),
    RemoveElement(Element),
    MoveElements {
        ids: Vec<ElementId>,
        delta: Point,
    },
    ModifyElement {
        before: Element,
        after: Element,
    },
}

pub struct HistoryStack {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    max_depth: usize, // Default: 100
}
```
