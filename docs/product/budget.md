# Simplicity & Performance Budget

This budget keeps Ruqa **simple and strong**: limits are set from measured values
plus a small margin. A change that exceeds a limit needs an ADR with measured
evidence and a reason. Numbers are recorded, not estimated; every entry names
the command or scenario that produced it.

## 1. Measurement Machine

| Item | Value |
|---|---|
| CPU | Intel Core i5-8250U @ 1.60 GHz (4 cores / 8 threads) |
| RAM | 15.4 GiB |
| OS | EndeavourOS (Arch), kernel 7.2.14-arch1-1 |
| Display | X11, 1600x1000, scale factor 1.0 (Xvfb for automated runs) |
| GPU | Intel UHD 620 (unused: rendering is CPU-only) |
| Rust | 1.99.1, release profile (`lto = true`, `codegen-units = 1`, `strip = true`) |
| Measured | 2026-11-03, commit `34d3735` |

All GUI numbers were measured by launching the real release binary in a virtual
X server (`Xvfb :99`), driving it with `xdotool` (XTEST), and inspecting the
resulting screenshots with ImageMagick.

## 2. Budget Table

| Metric | Measured (baseline) | Limit | Status |
|---|---|---|---|
| Direct dependencies — `ruqa-core` | 4 (`serde`, `serde_json`, `thiserror`, `tracing`) | 6 | OK |
| Direct dependencies — `ruqa-render` | 6 external (`parley`, `roughr`, `skrifa`, `tiny-skia`, `thiserror`, `tracing`) | 8 | OK |
| Direct dependencies — `ruqa-app` | 7 external (`winit`, `softbuffer`, `tiny-skia`, `thiserror`, `tracing`, `tracing-subscriber`) | 9 | OK |
| Unique crates in the workspace graph | 176 | 200 | OK |
| Release binary size (`ruqa-app`) | 5,891,160 B (5.62 MiB) | 7.0 MiB | OK |
| Cold start (process → mapped window) | 189 ms | 250 ms | OK (see note) |
| Cold start (process → presenter ready, in-app log) | 158 ms | 200 ms | OK |
| Idle CPU, no input, no dialog | 0.00 % (0 CPU ticks over 15 s) | 0.5 % | OK |
| Pixels changed while idle (15 s) | 0 | 0 | OK |
| Frames presented while idle (20 s session) | 2 total, both startup/dismiss | ≤ 3 | OK |
| Resident memory at idle | 19.5–22.3 MB | 100 MB | OK |
| Fixed benchmark frame time (2560x1440) | 225.5 ms median | 260 ms | OK (see note) |
| Longest source module | 349 LOC (`ruqa-core/src/element.rs`) | 400 LOC | OK |
| Longest test module | 572 LOC (`ruqa-app/tests/e2e_app_harness_tests.rs`) | 650 LOC | OK |
| Workspace tests | 298 passing | ≥ 298 | OK |
| `unsafe` code | 0 (all crates `#![deny(unsafe_code)]`) | 0 | OK |

Cold start note: the MVP success metric says "< 150 ms". Measured external time
is 189 ms and the in-app window/presenter handshake is 158 ms, so the claim is
currently **not met** and must not be repeated as fact. The 250 ms limit is a
regression guard, not a statement that the target is reached.

Fixed benchmark scene: 5,000 rough shapes (rect/ellipse/diamond/line/arrow mix),
2,000 smoothed freehand strokes (24 points each), 500 bilingual text boxes, all
inside the 2560x1440 viewport, rendered through
`WhiteboardApp::render_to_pixmap` with the dark grid enabled. That is a worst-case
stress ceiling: 225 ms is ~4.4 FPS. The 60 FPS goal in `docs/product/mvp.md`
applies to normal boards (tens of visible elements), not to this deliberately
dense scene. Culling and caching work is tracked separately; do not raise this
limit without an ADR.

## 3. Rules

1. Idle means idle: frames are rendered only for input, animation, or caret
   blink. No polling loop, no unconditional per-motion redraw.
2. Dependency additions need an existing issue or approved ADR before they land.
3. Module soft cap is ~350 LOC; the 400 LOC hard limit above applies to source
   modules (tests get 650 LOC).
4. Any regression beyond a limit needs an ADR with measured before/after numbers
   from this same scenario.
5. Re-measure on the machine above whenever `ruqa-render` or the event loop
   changes materially, and update this file in the same PR.
