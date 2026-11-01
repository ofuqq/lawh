# Lawh Documentation Index

Welcome to the architectural and product documentation for **Lawh** (لَوْح), a lightweight, open-source desktop whiteboard written in Rust.

---

## Directory Overview

```text
docs/
├── README.md                     # This documentation map
├── roadmap.md                    # Project roadmap and milestone exit criteria (M0–M5)
├── product/                      # Product strategy, vision, and scope
│   ├── vision.md                 # Long-term product mission and problem statement
│   ├── principles.md             # Core design principles (fast, offline, hand-drawn)
│   ├── users-and-use-cases.md    # Target personas, student workflows, and scenarios
│   ├── competitive-analysis.md   # Comparison against Excalidraw, tldraw, drafft-ink
│   └── mvp.md                    # Explicit MVP v0.1 requirements (MUST/SHOULD/WON'T)
├── architecture/                 # Technical design proposals (Draft status)
│   ├── overview.md               # High-level component decomposition
│   ├── data-model.md             # Element data model, scene graph, and math
│   ├── file-format.md            # .lawh versioned JSON schema & forward compatibility
│   ├── rendering.md              # 2D rendering pipeline (tiny-skia CPU & buffer presentation)
│   ├── input-and-tools.md        # Event handling, state machines, and mouse tools
│   └── testing-strategy.md       # Unit, integration, snapshot, and benchmark strategy
├── adr/                          # Architectural Decision Records (ADRs)
│   ├── README.md                 # ADR index and lifecycle explanation
│   ├── 0000-template.md          # Standard ADR template
│   ├── 0001-use-rust.md          # ADR 0001: Language Selection (Rust) [Accepted]
│   ├── 0002-license.md           # ADR 0002: Dual MIT / Apache-2.0 License [Accepted]
│   ├── 0003-rendering-backend.md # ADR 0003: CPU-First Rendering Pipeline [Proposed]
│   ├── 0004-file-format.md       # ADR 0004: Versioned JSON Document Format [Proposed]
│   └── 0005-project-name.md      # ADR 0005: Project Name & Namespace [Proposed]
├── business/                     # Open-source sustainability and governance
│   ├── open-source-model.md      # Permissive vs copyleft analysis, solo founder model
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
- **For Product & Scope Alignment**: Read [product/vision.md](product/vision.md) and [product/mvp.md](product/mvp.md).
- **For Architectural Decisions**: Read [adr/README.md](adr/README.md).
