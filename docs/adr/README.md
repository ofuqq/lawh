# Architecture Decision Records (ADRs)

Architecture Decision Records capture important architectural and technical choices made during the development of Ruqa, including the context, trade-offs, and consequences of each decision.

---

## ADR Index

| ADR | Title | Status | Date |
| :--- | :--- | :--- | :--- |
| [0001](0001-use-rust.md) | Use Rust as Core Implementation Language | **Accepted** | 2026-11-01 |
| [0002](0002-license.md) | Apache-2.0 Open Source License with NOTICE | **Accepted** | 2026-11-01 |
| [0003](0003-rendering-backend.md) | CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer) | **Accepted (Provisional until Spike 1)** | 2026-11-01 |
| [0004](0004-file-format.md) | Versioned JSON Document Format (.ruqa) | **Accepted** | 2026-11-01 |
| [0005](0005-project-name.md) | Project Naming & Renaming to Ruqa | **Accepted** | 2026-11-01 |
| [0006](0006-contingency-tauri-fallback.md) | Contingency Architecture: Pre-Approved Tauri v2 Hybrid Fallback | **Accepted** | 2026-11-01 |
| [0007](0007-text-layout-parley.md) | Text Layout & Arabic Typography via Linebender Parley | **Accepted (Provisional until Spike 2)** | 2026-11-01 |
| [0008](0008-ui-chrome-toolkit.md) | UI Chrome Toolkit Evaluation (egui vs Custom CPU Chrome) | **Proposed** | 2026-11-01 |
| [0009](0009-ai-assisted-development.md) | AI-Assisted Development & Governance Policy | **Accepted** | 2026-11-01 |

---

## ADR Lifecycle

- **Proposed**: Under review by the project owner. Not yet binding.
- **Accepted**: Approved by the project owner and currently enforced.
- **Accepted (Provisional)**: Approved directionally, subject to empirical validation in a time-boxed technical spike.
- **Superseded**: Replaced by a newer ADR.
- **Rejected**: Evaluated and discarded.
