# Founder Decision Log

This log records pivotal product, technical, and governance decisions made by the repository owner. It preserves historical context and explains the rationale behind major choices.

---

### [2026-11-01] Decision 1: Canonical Repository & Organization Name
- **Decision**: Adopt `ofuqq/lawh` as the canonical repository name.
- **Context**: The GitHub account `Afaq` is an inactive user account registered in 2012. The founder owns the GitHub organization `ofuqq` (named "Ofuq", Arabic: أفق).
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 2: Dual MIT / Apache-2.0 License
- **Decision**: Dual-license Lawh under MIT OR Apache-2.0.
- **Context**: Evaluated against copyleft (AGPL-3.0). While AGPL protects against proprietary SaaS wrappers, Lawh is an offline desktop tool. Permissive dual licensing aligns with the broader Rust ecosystem and maximizes adoption of `lawh-core`.
- **Status**: **Accepted** (recorded in ADR 0002).

---

### [2026-11-01] Decision 3: Code of Conduct Enforcement Contact
- **Decision**: Designate `ahossam.tech@gmail.com` as the official reporting email for Code of Conduct enforcement.
- **Context**: Direct founder communication channel for prompt handling of community reports.
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 4: Arabic Documentation Scope
- **Decision**: Maintain `README.ar.md` as an Arabic summary in the public repository; all public architectural, development, and ADR documentation will be in English. The founder's private workflow guide (`.local/GUIDE.ar.md`) will be in Egyptian Arabic.
- **Context**: Balances international open-source reach with the project's Arabic roots.
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 5: Contributor Agreement Policy (DCO / CLA)
- **Decision**: Do not require DCO (`Signed-off-by`) or CLA for initial development. Contributions are accepted inbound=outbound under the project license.
- **Trigger to Revisit**: **Revisit DCO enforcement when the first external non-founder contributor arrives**.
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 6: CPU-First Rendering Strategy
- **Decision**: Use CPU rasterization via `tiny-skia` with window buffer presentation via `softbuffer` and stable `winit 0.30.x`.
- **Context**: Avoids GPU driver incompatibilities on Linux desktop environments and CI runners. Architecture will isolate rendering in a separate crate without building an unnecessary speculative trait hierarchy.
- **Status**: **Proposed & In Progress** (recorded in ADR 0003).

---

### [2026-11-01] Decision 7: Data-Driven GPU Migration Benchmark
- **Decision**: Add an automated stroke benchmark (1,000, 5,000, and 10,000 strokes) as an exit criterion for Milestone M1 before evaluating GPU rasterization (`vello`).
- **Context**: Prevents premature optimization. Software rendering is evaluated against real frame times (< 16.6 ms target at 1k strokes) before introducing complex GPU compute shaders.
- **Status**: **Approved & Implemented** (recorded in `docs/product/mvp.md` and `docs/roadmap.md`).
