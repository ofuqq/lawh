# Project & Technical Glossary

A plain-language glossary explaining key Rust concepts, graphics rendering terms, and whiteboard mechanics used throughout Lawh.

---

## 1. Rust Ecosystem Terminology

- **Crate**: The smallest compilation unit in Rust (either an executable binary or a reusable library).
- **Workspace**: A set of related crates (e.g., `lawh-core`, `lawh-render`, `lawh-app`) that share a single `Cargo.lock` file, unified target directory, and common dependency versions.
- **Ownership & Borrowing**: Rust's memory management model. Every value has a single owner. Values can be borrowed immutably (`&T`) by many readers, or mutably (`&mut T`) by exactly one writer, preventing data races at compile time.
- **Lifetimes (`'a`)**: Named parameters used by the compiler to ensure references remain valid for as long as data exists, avoiding dangling pointers.
- **Trait**: An interface contract defining shared behavior that types can implement (e.g., `Serialize`, `Clone`, `Debug`).
- **Enum with Data**: Rust enums are algebraic sum types that can encapsulate different associated data payloads per variant (e.g., `Element::Rectangle` vs `Element::Stroke`).
- **Clippy**: Rust’s official linter that catches common mistakes, performance bottlenecks, and non-idiomatic patterns.
- **Rustfmt**: The official deterministic code formatting tool for Rust.
- **Serde**: Rust’s framework for serializing and deserializing data structures efficiently between Rust types and data formats like JSON.

---

## 2. Whiteboard & Graphics Domain Terminology

- **Infinite Canvas**: A 2D drawing plane without rigid horizontal or vertical page boundaries, allowing users to draw and zoom outward indefinitely.
- **World Coordinates**: The fixed coordinate space of the canvas where drawn shapes actually reside.
- **Screen Coordinates**: The pixel coordinates of the operating system window on your display.
- **Viewport**: The rectangular window looking onto a portion of the world canvas, defined by an $(x, y)$ pan offset and a zoom scalar.
- **AABB (Axis-Aligned Bounding Box)**: The smallest rectangle with edges parallel to the coordinate axes that encloses a shape. Used for fast collision detection and viewport culling.
- **Hit-Testing**: The algorithm that checks whether a user's mouse pointer intersects with a shape on the canvas.
- **Rasterization**: The mathematical process of converting vector paths (lines, curves, points) into colored pixels on a grid.
- **Pixmap**: An in-memory grid of colored pixels (typically 32-bit RGBA).
- **Softbuffer**: A lightweight Rust library that presents an in-memory CPU pixel buffer directly to a native OS window without needing OpenGL or Vulkan.
- **Rough Algorithm**: A procedural algorithm (originally Rough.js) that creates an organic, hand-drawn look by perturbing line endpoints with calculated jitter and drawing multi-pass sketchy strokes.
- **Velocity-Based Smoothing**: Simulating pen pressure by calculating the speed of mouse movement—drawing thinner ink during fast gestures and thicker ink during slow, deliberate movements.
