# ADR 0006: Contingency Architecture Plan (Tauri v2 Hybrid Fallback)

- **Status**: **Accepted** (Contingency)
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Ruqa's primary architecture is a pure native Rust application (`ruqa-core`, `ruqa-render`, `ruqa-app`) targeting zero web-runtime overhead, sub-25 MB RAM consumption, and sub-second cold boot.
However, building a complete 2D desktop graphics stack (windowing, CPU/GPU rasterization, hand-drawn stroke geometry, and internationalized BiDi text editing) in Rust is technically demanding.

To de-risk the project against indefinite stall or failure during initial prototyping, a formal, pre-approved contingency architecture is required.

---

## 2. Trigger Condition (The Day-14 Gate)

This contingency plan is automatically triggered if **Gate 1 (Day-14 Technical Spike)** fails:
- **Deadline**: 14 calendar days from the start of Milestone M0 spikes.
- **Trigger Conditions**:
  1. Failure of Spike 1: Unable to achieve smooth 60 FPS rendering of 1,000 rough shapes + freehand strokes via `tiny-skia` + `softbuffer` on standard integrated CPU graphics (x86_64 / aarch64 Linux/macOS).
  2. Failure of Spike 2: Inability to render basic mixed Arabic-English text and cursor hit-testing reliably using `parley` within the prototype window.
  3. Window / Event Loop Deadlock: Unresolvable `winit` event loop or input dispatch bugs consuming more than 40 engineering hours without a working interactive canvas.

If Gate 1 fails, the founder immediately executes the pivot without reopening architectural debates.

---

## 3. Contingency Architecture: Tauri v2 Hybrid

If triggered, Ruqa pivots to a **Tauri v2** hybrid application:

```
+--------------------------------------------------------------+
|               Frontend (Webview Canvas / UI)                 |
|  - Canvas 2D / WebGL rendering layer                         |
|  - Rough.js / Perfect-freehand (MIT libraries)               |
|  - Browser-native BiDi / HarfBuzz text shaping & input IME   |
+--------------------------------------------------------------+
                               |
                   IPC / Shared Memory Bridge
                               |
+--------------------------------------------------------------+
|               Backend (Rust Core & Local Daemon)             |
|  - ruqa-core (Document scene graph, JSON serialization)    |
|  - Vault storage, file system watchers, auto-save            |
|  - Export utilities (PNG/SVG export via headless renderer)   |
|  - Native OS dialogs, menu integration                       |
+--------------------------------------------------------------+
```

### Clean-Room Constraints for Frontend:
- Must use strictly permissive components (MIT, Apache-2.0, or BSD).
- Never copy code from AGPL/GPL projects (e.g., drafft-ink, Rnote).
- Retain existing `.ruqa` JSON schema and `ruqa-core` Rust crate for document state and file I/O.

---

## 4. Consequences

### Positive:
- **Instant Browser Text Engine**: Resolves complex BiDi, HarfBuzz shaping, cursive font fallback, and OS IME input out of the box using the system webview.
- **De-risks Schedule**: Guarantees a working desktop MVP within a realistic timeline even if pure native Rust windowing/text layout stalls.
- **Reuses Core Logic**: The entire `ruqa-core` crate (data model, file format, undo/redo, validation) remains 100% native Rust and unchanged.

### Negative:
- **Memory Footprint**: System webview runtime increases baseline idle RAM consumption from ~20 MB to ~80–120 MB.
- **Platform Inconsistencies**: Webview rendering engines differ between Linux (WebKitGTK), Windows (WebView2 / Chromium), and macOS (WebKit).
- **Secondary Transition**: Requires rewiring event handling across the Tauri IPC boundary.
