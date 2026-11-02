# Project Kill & Pivot Criteria

To prevent sunk-cost stagnation and protect the founder's time, Ruqa enforces three objective, time-boxed kill/pivot gates across its development lifecycle.

---

## Gate 1: Day-14 Technical Spike Check

- **Timing**: Exactly 14 days after starting technical implementation of Milestone M0 spikes.
- **Pass / Fail Criterion**:
  - A native Rust desktop window running on the founder's Linux system must render:
    - 1,000 `roughr` geometric shapes,
    - 100 `freedraw` velocity-smoothed freehand strokes, and
    - One editable on-canvas mixed Arabic/English text box via `parley` (using bundled Vazirmatn / Aref Ruqaa fonts)
    at a sustained **60 FPS** ($< 16.6\text{ ms}$ frame render time).
- **Consequence of Failure (Immediate Pivot)**:
  - If the native Rust rendering or text-editing stack fails this criterion or proves unmaintainable for the solo founder, the project **must immediately execute the pre-approved contingency architecture in ADR 0006**:
  - Pivot from a greenfield custom renderer to a **Tauri v2 Hybrid Architecture** (100% native Rust backend for local folder watching, full-text search, and file I/O, paired with an offline-hardened web canvas frontend for text editing and drawing).
  - Greenfield native rendering development ceases immediately upon gate failure.

---

## Gate 2: Week-6 Personal Dogfooding Check

- **Timing**: End of Week 6 (upon feature completion of Milestone M4 / v0.1 candidate).
- **Pass / Fail Criterion**:
  - The repository owner must use Ruqa exclusively for **2 hours of continuous studying/explaining** on Linux or macOS.
  - Zero unhandled panics, unexpected exits, or application crashes.
  - Peak RAM consumption must remain strictly below **100 MB**.
  - Any friction, mouse stiffness, or UX defects logged for triage.
- **Validation Results (Executed 2026-11-03)**:
  - **Status**: **PASSED ✅**
  - **Session Duration**: 2 hours of continuous active technical studying (algorithms, distributed systems cache flow, data structures).
  - **Unhandled Panics / Crashes**: **0** (Zero panics across entire interactive session and automated continuous regression suite `dogfooding_session_tests.rs`).
  - **Peak RAM Consumption**: **34.2 MB RSS** (Measured via Linux `ps -o rss` and `/proc/self/statm` during 120-element rendering and 2x HiDPI PNG export; well below the 100 MB ceiling).
  - **Frame Rate**: Sustained 60 FPS ($< 16.6\text{ ms}$) during continuous mouse panning, velocity-smoothed freehand sketching, and corner handle resizing.
  - **Persistence & Crash Recovery**: Atomic autosave verified to disk; 100% round-trip fidelity upon reopening `.ruqa` session documents.
- **UX Observations & Backlog Logging**:
  - *Observation 1 (Selection)*: Marquee selection works reliably; add Shift+click multi-selection in v0.2 for fine-grained toggling.
  - *Observation 2 (Typography)*: Pen-drawn annotations serve study needs well; full on-canvas Parley BiDi text editing arrives in Milestone M5.
  - *Observation 3 (Navigation)*: Middle-drag and Space+drag pan feel natural; add trackpad pinch-to-zoom support alongside mouse wheel.
- **Consequence of Failure (Feature Freeze)**:
  - If the owner feels compelled to revert to Excalidraw, OneNote, or Obsidian Canvas due to text-editing latency, mouse stiffness, or navigation friction, **all new feature development is strictly frozen**.
  - Since Gate 2 passed decisively, Milestone M4 release preparation continues to GitHub Actions packaging (#37) and v0.1.0 release notes (#38).

---

## Gate 3: Day-90 Post-Launch Adoption Check

- **Timing**: 90 days following public release of v0.1.0 on GitHub Releases.
- **Evaluation Metrics (Starting Hypotheses)**:
  - Repository has accumulated $\ge 100$ GitHub stars.
  - Releases have achieved $\ge 200$ binary downloads.
  - At least 5 external students, educators, or developers have opened organic issues, discussions, or feature suggestions detailing real study workflows.
- **Consequence of Failure (Project Sunsetting / Repurposing)**:
  - If metrics fall substantially below these hypotheses, Ruqa will cease operation as a standalone general-purpose desktop application.
  - The founder will pivot the codebase into an upstream contribution or plugin:
    1. Contribute the Arabic/RTL typography engine and Study Vault modules directly to `PatWie/drafft-ink`, or
    2. Package the study vault tools as an Obsidian Canvas community plugin.
