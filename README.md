# Ruqa

> [!WARNING]
> **Status: Pre-Alpha / In Active Bootstrapping**  
> Ruqa is in early foundational development. The architecture, file format, and core crates are currently being established. Functional binaries and public releases will begin with milestone `v0.1.0`.

[![CI](https://github.com/ofuqq/ruqa/actions/workflows/ci.yml/badge.svg)](https://github.com/ofuqq/ruqa/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

[Architecture](docs/README.md) | [Roadmap](docs/roadmap.md) | [Kill Criteria](docs/product/kill-criteria.md) | [Contributing](CONTRIBUTING.md)

---

**Ruqa** (named after Ruq'ah, the swift, clean Arabic handwriting style) is a lightweight, open-source desktop whiteboard and study scratchpad built in Rust by [Ofuq](https://github.com/ofuqq).

Inspired by tools like Excalidraw, Ruqa is engineered from scratch for **studying, teaching, and visual explaining**. It offers an aesthetic hand-drawn sketch style, buttery-smooth pen ink, an infinite zoomable canvas, a default dark theme, and complete offline privacy—operable entirely with a standard mouse.

---

## Key Highlights

- **Hand-Drawn Aesthetic**: Rough, sketchy geometric shapes (rectangles, ellipses, arrows, lines, diamonds) that keep diagrams approachable and informal.
- **Smooth Freehand Pen**: Velocity-simulated, responsive ink strokes designed to feel natural even with mouse input.
- **Infinite Canvas**: Smooth 60+ FPS panning and zooming without arbitrary workspace boundaries.
- **Dark Theme by Default**: High-contrast, easy-on-the-eyes palette tailored for late-night study and recording video lectures.
- **Mouse-First Usability**: Every tool, selection, and canvas manipulation is fluidly operable using a two-button mouse with wheel.
- **Offline & Private**: Self-contained local files with zero telemetry, zero mandatory cloud accounts, and instant startup (< 500 ms target).
- **Long-Term Arabic/RTL Vision**: Built from day one with architectural readiness for first-class bidirectional Arabic text layout (scheduled for Milestone M5).

---

## Planned Features (v0.1.0 MVP)

- [ ] **Canvas Foundation**: Infinite panning (middle-click drag / space drag) and smooth zooming centered on cursor.
- [ ] **Drawing Tools**: Freehand pen, line, arrow, rectangle, ellipse, and diamond.
- [ ] **Styling**: Hand-drawn stroke jitter, fill patterns, stroke color, and stroke width.
- [ ] **Direct Manipulation**: Select, translate, resize, duplicate, and delete shapes.
- [ ] **History**: Unlimited local undo and redo stack.
- [ ] **Persistence**: Single-file versioned JSON format (`.ruqa`) with forward compatibility.
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
git clone https://github.com/ofuqq/ruqa.git
cd ruqa

# Check code formatting (Tested in CI)
cargo fmt --all --check

# Run linter checks (Tested in CI)
cargo clippy --workspace --all-targets -- -D warnings

# Run test suite (Tested in CI)
cargo test --workspace

# Run license and dependency verification (Tested in CI)
cargo deny check licenses bans sources

# Run the desktop application (Untested - Windowing runtime under M1)
cargo run -p ruqa-app
```

> [!NOTE]
> Local desktop execution commands (`ruqa-app`) are marked as **untested** until the Milestone M1 windowing harness lands. Workflows and tests are verified via GitHub Actions CI on Linux runners.

---

## Project Structure

```text
ruqa/
├── crates/
│   ├── ruqa-core/         # Core geometry, element models, undo stack, and file I/O
│   ├── ruqa-render/       # (M1) 2D rendering pipeline (tiny-skia CPU, roughr, softbuffer)
│   └── ruqa-app/          # (M1) Native OS windowing, mouse event loop, and UI chrome
├── docs/                  # Architectural specs, product vision, ADRs, and guides
│   ├── adr/               # Architecture Decision Records (0001–0009)
│   ├── architecture/      # Detailed design documents
│   ├── business/          # Open-source sustainability and decision logs
│   ├── development/       # Contributor setup and AI agent workflow
│   ├── product/           # Vision, principles, competitive analysis, and kill gates
│   └── research/          # Market research digest and crate audits
├── .github/               # CI workflows, issue templates, PR template, rulesets
├── LICENSE                # Apache License, Version 2.0
├── NOTICE                 # Attribution and copyright notice
├── deny.toml              # Cargo-deny license and ban policy
└── README.md
```

---

## Documentation & Architecture

- **[Product Vision & Principles](docs/product/vision.md)**: Why Ruqa exists and the principles guiding its development.
- **[MVP Scope & Non-Goals](docs/product/mvp.md)**: Explicit scope boundaries for the v0.1 release.
- **[Kill Criteria & Gates](docs/product/kill-criteria.md)**: Empirical go/no-go gates for technical spikes and dogfooding.
- **[Architectural Overview](docs/architecture/overview.md)**: Modular design separating math/core, rendering, and windowing.
- **[Architecture Decision Records (ADRs)](docs/adr/README.md)**: Index of architectural choices from language to contingency fallbacks.

---

## Contributing

We welcome community feedback, code contributions, and discussions. Please read our [Contributing Guide](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting pull requests.

---

## License

Ruqa is open source licensed under the **Apache License, Version 2.0**.  
See the [LICENSE](LICENSE) file and [NOTICE](NOTICE) for details.
