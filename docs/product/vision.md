# Product Vision: Lawh (لَوْح)

## 1. Problem Statement

Modern digital whiteboard tools have increasingly drifted toward heavy, enterprise-focused, collaborative web platforms (Miro, Mural, FigJam, tldraw cloud). While these platforms excel at team sprint retrospectives and corporate workshops, they introduce significant friction for individual learners, educators, and technical explainers:

1. **Heavy Resource Footprint**: Running Chromium/Electron instances with complex web bundles consumes hundreds of megabytes of memory and battery life, causing noticeable input latency on budget hardware.
2. **Cloud & Account Dependency**: Forced account creation, telemetry, paywalled export resolutions, and mandatory internet connectivity disrupt focused solo study sessions and offline work.
3. **Rigid, Overly Formal Aesthetics**: Standard vector tools produce sterile, overly rigid shapes that make rapid idea-sketching feel intimidating rather than exploratory.
4. **Neglected Right-to-Left (RTL) Support**: Existing whiteboard tools treat Arabic text as an afterthought, leading to broken bidirectional shaping, reversed cursor movements, and misalignment with diagram flow.

## 2. The Lawh Mission

**Lawh** (Arabic for *board* or *slate*) is a fast, lightweight, open-source desktop whiteboard designed specifically for **studying, self-explaining, and technical communication**.

It provides an approachable, hand-drawn aesthetic where imperfect sketches communicate concepts faster than polished diagrams. By running natively via a lean Rust runtime, Lawh launches instantly, consumes minimal system resources, works completely offline, and responds immediately to mouse input.

## 3. Core Pillars

- **Zero-Friction Solo Study**: Launch in under 100 milliseconds, open a local `.lawh` file, and sketch immediately. No sign-ups, no sync spinners, no subscriptions.
- **The Hand-Drawn Advantage**: By introducing intentional micro-variations and sketchy contours to geometric shapes, Lawh removes the psychological pressure of visual perfection, fostering freer thinking and faster diagramming.
- **Mouse-First Fluidity**: Not everyone owns a drawing tablet or stylus. Lawh’s velocity-sensitive stroke algorithms and intuitive mouse bindings (middle-drag panning, scroll zooming, edge handles) ensure desktop mouse users draw clean notes effortlessly.
- **First-Class Arabic & RTL Foundation**: While full text shaping is scheduled post-v0.1, Lawh is architecturally architected to treat Right-to-Left text and canvas semantics as a core citizen, not a third-party hack.
