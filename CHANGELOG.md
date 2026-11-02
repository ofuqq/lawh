# Changelog

All notable changes to Ruqa will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Selection tool interaction in `ruqa-app` supporting single-click select, Shift-click toggle, empty canvas deselect, and drag-marquee intersection selection (fixes #19).
- Semi-transparent selection marquee overlay rasterization (`render_marquee`) in `ruqa-render` (fixes #19).
- Precise point and boundary distance hit-testing (`Element::hit_test`, `Scene::hit_test_topmost`) in `ruqa-core` distinguishing filled interiors from stroke perimeters (fixes #18).
- Tight axis-aligned bounding box calculation (`Element::bounding_box`) in `ruqa-core` with stroke width padding, arrowhead barb geometry, and rectangle inflation (fixes #17).
- Freehand variable-width ink polygon rasterizer in `ruqa-render` with rounded end caps and zero-allocation scratch buffers for 60 FPS drawing (fixes #16).
- Freehand pen stroke smoothing in `ruqa-core` via Catmull-Rom spline interpolation and pointer velocity pressure modulation (fixes #15).
- Hand-drawn sketchy geometric shape renderer (`Rectangle`, `Ellipse`, `Diamond`, `Line`, `Arrow`) in `ruqa-render` with stable seed reproducibility and viewport culling (fixes #14).
- Core `Scene` model in `ruqa-core` with monotonic element IDs, z-index reordering, spatial aggregate bounds, and viewport visibility culling (fixes #50).
- Geometric shape primitives (`Rectangle`, `Ellipse`, `Diamond`, `Line`, `Arrow`), `ElementStyle`, and `FillStyle` in `ruqa-core` with Serde JSON round-trip serialization (fixes #13).
- Converter from `roughr` operation sets and drawables to `tiny_skia::Path` with open/closed path conversion and round stroke capping (fixes #12).
- Pan and zoom canvas interaction in `ruqa-app` via middle-mouse drag, Space + left drag, and cursor-anchored wheel zooming with phantom-click suppression (fixes #11).
- 2D canvas viewport camera and coordinate transforms in `ruqa-core` with anchored zooming and visible rect calculation (fixes #10).
- Desktop window initialization, softbuffer frame presentation, and HiDPI coordinate scaling with headless smoke-test harness (fixes #8, #9, #49, #47).
- Initial project bootstrap, final naming as Ruqa (from Ruq'ah, the Arabic handwriting style), and governance framework.
- Apache-2.0 licensing with NOTICE and `cargo-deny` policy enforcement.
- Product documentation: vision, principles, users and use cases, competitive analysis, MVP scope, and kill criteria gates.
- Technical architecture specifications: 3-crate layout (`ruqa-core`, `ruqa-render`, `ruqa-app`), data model, file format (`.ruqa`), CPU rendering pipeline, text/RTL shaping via Parley, and testing strategy.
- Architectural Decision Records: ADR 0001 through ADR 0009.
- Market and technical research synthesis digest (`docs/research/`).
- GitHub Actions CI with pinned SHAs: format, clippy, unit tests, and `cargo-deny` license/vulnerability audit.
- GitHub issue templates (bug, feature, task, spike), PR template with AI disclosure, labels taxonomy, and main branch ruleset.
- Minimal Rust workspace skeleton with `ruqa-core` crate and baseline unit tests.
