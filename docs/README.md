# Ruqa Documentation Index

Welcome to the architectural, product, and engineering documentation for **Ruqa** (Ruq'ah), a lightweight, open-source desktop whiteboard and study scratchpad written in Rust.

---

## Directory Overview

```text
docs/
├── README.md                     # This documentation map
├── roadmap.md                    # Project roadmap and milestone exit criteria (M0–M5)
├── research/                     # Technical and market research digests
│   └── 2026-11-market-and-technical-research.md # Market analysis, crate audit, source index
├── product/                      # Product strategy, vision, and scope
│   ├── vision.md                 # Long-term product mission and problem statement
│   ├── principles.md             # Core design principles (fast, offline, hand-drawn)
│   ├── users-and-use-cases.md    # Target user roles, study workflows, and scenarios
│   ├── competitive-analysis.md   # Clean-room comparison against Excalidraw, tldraw, drafft-ink
│   ├── mvp.md                    # Explicit MVP v0.1 scope, v0.2 scope, and strict WON'Ts
│   └── kill-criteria.md          # Formal go/no-go gates (Day-14, Week-6, Day-90)
├── architecture/                 # Technical design proposals
│   ├── overview.md               # High-level 3-crate architecture decomposition
│   ├── data-model.md             # Scene graph, elements, geometry, and spatial index
│   ├── file-format.md            # .ruqa versioned JSON schema & forward compatibility
│   ├── rendering.md              # 2D rendering pipeline (tiny-skia CPU, roughr, softbuffer)
│   ├── text-and-rtl.md           # Text layout, BiDi, shaping (parley/harfrust), fonts
│   ├── input-and-tools.md        # Event handling, state machines, and mouse tools
│   └── testing-strategy.md       # Unit, integration, snapshot, and benchmark strategy
├── adr/                          # Architectural Decision Records (ADRs)
│   ├── README.md                 # ADR index and lifecycle explanation
│   ├── 0000-template.md          # Standard ADR template
│   ├── 0001-use-rust.md          # ADR 0001: Language Selection (Rust) [Accepted]
│   ├── 0002-license.md           # ADR 0002: Apache-2.0 License with NOTICE [Accepted]
│   ├── 0003-rendering-backend.md # ADR 0003: CPU-First Rendering Pipeline [Accepted, Provisional]
│   ├── 0004-file-format.md       # ADR 0004: Versioned JSON Document Format [Accepted]
│   ├── 0005-project-name.md      # ADR 0005: Project Naming & Renaming to Ruqa [Accepted]
│   ├── 0006-contingency-tauri-fallback.md # ADR 0006: Contingency Tauri v2 Fallback [Accepted]
│   ├── 0007-text-layout-parley.md # ADR 0007: Text Layout & Shaping via Parley [Accepted, Provisional]
│   ├── 0008-ui-chrome-toolkit.md # ADR 0008: UI Chrome Toolkit Evaluation [Proposed]
│   └── 0009-ai-assisted-development.md # ADR 0009: AI-Assisted Development Policy [Accepted]
├── business/                     # Open-source sustainability and governance
│   ├── open-source-model.md      # Permissive vs copyleft analysis, zero-monetization v0.1
│   └── decision-log.md           # Chronological record of major founder decisions
└── development/                  # Engineering processes and guides
    ├── setup.md                  # Local toolchain installation and build instructions
    ├── workflow.md               # Git branching, Conventional Commits, review cadence
    ├── ai-assisted-development.md# Collaboration guidelines for AI agents
    └── glossary.md               # Plain-language Rust and whiteboard domain glossary
```

---

## Where to Begin

- **For AI Agents & Contributors**: Read [AGENTS.md](../AGENTS.md) first, then review [development/workflow.md](development/workflow.md) and [architecture/overview.md](architecture/overview.md).
- **For Product & Scope Alignment**: Read [product/vision.md](product/vision.md), [product/mvp.md](product/mvp.md), and [product/kill-criteria.md](product/kill-criteria.md).
- **For Architectural Decisions**: Read [adr/README.md](adr/README.md).
