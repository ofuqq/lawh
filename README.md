# Lawh (لَوْح)

> [!WARNING]
> **Status: Pre-Alpha / In Active Bootstrapping**  
> Lawh is in early foundational development. The architecture, file format, and crates are currently being established. Public releases and functional binaries will begin with milestone `v0.1.0`.

[![CI](https://github.com/ofuqq/lawh/actions/workflows/ci.yml/badge.svg)](https://github.com/ofuqq/lawh/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

[العربية (Arabic Summary)](README.ar.md) | [Architecture](docs/README.md) | [Roadmap](docs/roadmap.md) | [Contributing](CONTRIBUTING.md)

---

**Lawh** (Arabic: لَوْح, meaning *board* or *tablet*) is a lightweight, open-source desktop whiteboard built in Rust by [Ofuq](https://github.com/ofuqq) (أفق).

Inspired by tools like Excalidraw, Lawh is designed from scratch for **studying, teaching, and visual explaining**. It offers an aesthetic hand-drawn sketch style, buttery-smooth pen ink, an infinite zoomable canvas, a default dark theme, and complete offline privacy—operable entirely with a standard mouse.

---

## Key Highlights

- **Hand-Drawn Aesthetic**: Rough, sketchy geometric shapes (rectangles, ellipses, arrows, lines) that keep diagrams approachable and informal.
- **Smooth Freehand Pen**: Pressure-simulated, responsive ink strokes designed to feel natural even with mouse input.
- **Infinite Canvas**: Smooth 60+ FPS panning and zooming without arbitrary workspace boundaries.
- **Dark Theme by Default**: High-contrast, easy-on-the-eyes palette tailored for late-night study and recording video lectures.
- **Mouse-First Usability**: Every tool, selection, and canvas manipulation is fluidly operable using a two-button mouse with wheel.
- **Offline & Private**: Self-contained local files with zero telemetry, zero mandatory cloud accounts, and instant startup.
- **Long-Term Arabic/RTL Vision**: Built from day one with architectural readiness for first-class bidirectional Arabic text layout (planned for post-v0.1).

---

## Planned Features (v0.1.0 MVP)

- [ ] **Canvas Foundation**: Infinite panning (middle-click drag / space drag) and smooth zooming centered on cursor.
- [ ] **Drawing Tools**: Freehand pen, line, arrow, rectangle, and ellipse.
- [ ] **Styling**: Hand-drawn stroke jitter, fill patterns, stroke color, and stroke width.
- [ ] **Direct Manipulation**: Select, translate, resize, duplicate, and delete shapes.
- [ ] **History**: Unlimited local undo and redo stack.
- [ ] **Persistence**: Single-file versioned JSON format (`.lawh`) with forward compatibility.
- [ ] **Export**: Clean PNG snapshot export for sharing and study notes.

See [docs/product/mvp.md](docs/product/mvp.md) and the [Product Roadmap](docs/roadmap.md) for detailed milestone criteria.

---

## Quickstart & Build Instructions

### Prerequisites
- Rust stable (1.80+ recommended). Install via [rustup](https://rustup.rs):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Building & Running
```bash
# Clone the repository
git clone https://github.com/ofuqq/lawh.git
cd lawh

# Check code formatting (Tested in CI)
cargo fmt --check

# Run linter checks (Tested in CI)
cargo clippy --workspace --all-targets -- -D warnings

# Run test suite (Tested in CI)
cargo test --workspace

# Run the desktop application (Untested - Windowing runtime under M1)
cargo run -p lawh-app
```

> [!NOTE]
> Local desktop execution commands are marked as **untested** until the M1 windowing harness lands. Workflows are verified via GitHub Actions CI on Linux runners.

---

## Project Structure

```text
lawh/
├── crates/
│   └── lawh-core/         # Core data structures, canvas math, and model
├── docs/                  # Architectural specs, product vision, ADRs, and guides
│   ├── adr/               # Architecture Decision Records
│   ├── architecture/      # Detailed design documents
│   ├── business/          # Open-source sustainability and decision logs
│   ├── development/       # Contributor setup and AI agent workflow
│   └── product/           # Vision, principles, competitive analysis, and MVP
├── .github/               # CI workflows, issue templates, PR template, rulesets
└── README.md
```

---

## Documentation & Architecture

- **[Product Vision & Principles](docs/product/vision.md)**: Why Lawh exists and the principles guiding its development.
- **[MVP Scope & Non-Goals](docs/product/mvp.md)**: Explicit scope boundaries for the v0.1 release.
- **[Architectural Overview](docs/architecture/overview.md)**: Modular design separating math/core, rendering, and windowing.
- **[Architecture Decision Records (ADRs)](docs/adr/README.md)**:
  - [ADR 0001: Language Selection (Rust)](docs/adr/0001-use-rust.md)
  - [ADR 0002: Dual MIT / Apache-2.0 License](docs/adr/0002-license.md)
  - [ADR 0003: CPU-First Rendering with tiny-skia](docs/adr/0003-rendering-backend.md)
  - [ADR 0004: File Format Specification](docs/adr/0004-file-format.md)
  - [ADR 0005: Project Name & Crates Namespace](docs/adr/0005-project-name.md)

---

## Contributing

We welcome community feedback, code contributions, and discussions. Please read our [Contributing Guide](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting pull requests.

---

## License

Lawh is open source and dual-licensed under either:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
