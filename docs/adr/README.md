# Architecture Decision Records (ADRs)

Architecture Decision Records capture important architectural and technical choices made during the development of Lawh, including the context, trade-offs, and consequences of each decision.

---

## ADR Index

| ADR | Title | Status | Date |
| :--- | :--- | :--- | :--- |
| [0001](0001-use-rust.md) | Use Rust as Core Implementation Language | **Accepted** | 2026-11-01 |
| [0002](0002-license.md) | Dual MIT / Apache-2.0 Open Source License | **Accepted** | 2026-11-01 |
| [0003](0003-rendering-backend.md) | CPU-First 2D Rendering Pipeline (tiny-skia + softbuffer) | **Proposed** | 2026-11-01 |
| [0004](0004-file-format.md) | Versioned JSON Document Format (.lawh) | **Proposed** | 2026-11-01 |
| [0005](0005-project-name.md) | Project Naming & Crates Ecosystem Namespace | **Proposed** | 2026-11-01 |

---

## ADR Lifecycle

- **Proposed**: Under review by the project owner. Not yet binding.
- **Accepted**: Approved by the project owner and currently enforced.
- **Superseded**: Replaced by a newer ADR (e.g., if a Proposed GPU backend replaces CPU rendering).
- **Rejected**: Evaluated and discarded.
