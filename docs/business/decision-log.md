# Founder Decision Log

This log records major technical, architectural, product, and governance decisions made by the repository owner.

---

### [2026-11-01] Decision 1: Rename Project from Lawh to Mirsam
- **Decision**: Adopt **Mirsam** (`ofuqq/mirsam`, crates `mirsam-core`, `mirsam-render`, `mirsam-app`) as canonical project and repository name.
- **Context**: External research report identified an active commercial company, Lawh.ai (https://lawh.ai/features), operating in the exact same Arabic/RTL vector canvas vertical. Renaming avoids trademark infringement risk (Classes 9/42) and SEO dilution. Audited candidate list: `mirsam` is clean on crates.io (404), clean on GitHub (`ofuqq/mirsam`), and has no software whiteboard collision.
- **Status**: **Approved & Implemented** (recorded in ADR 0005).

---

### [2026-11-01] Decision 2: License Switch to Apache-2.0 ONLY
- **Decision**: License Mirsam strictly under **Apache-2.0** with a `NOTICE` file; remove MIT dual-licensing. Enforce license compliance via `cargo-deny` in CI.
- **Context**: Apache-2.0 provides an explicit patent grant and strong contributor protections while remaining widely compatible across the Rust ecosystem.
- **Status**: **Accepted** (recorded in ADR 0002).

---

### [2026-11-01] Decision 3: Clean-Room Implementation & Rejection of drafft-ink Fork
- **Decision**: Build Mirsam clean-room; strictly prohibit copying code from AGPL/GPL projects (drafft-ink, Rnote, Xournal++). Reject forking drafft-ink due to AGPLv3 copyleft incompatibility with Apache-2.0.
- **Context**: Preserves license purity and developer trust.
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 4: 3-Crate Architecture & Pre-Approved Tauri Fallback
- **Decision**: Scaffold greenfield 3-crate architecture (`mirsam-core`, `mirsam-render`, `mirsam-app`). Pre-approve a contingency fallback to a Tauri v2 hybrid architecture if the Day-14 Technical Spike fails.
- **Context**: Decouples domain models from rendering while providing an immediate escape hatch if native Rust text editing stalls.
- **Status**: **Accepted** (recorded in ADR 0006).

---

### [2026-11-01] Decision 5: CPU-First Rendering Pipeline (tiny-skia + softbuffer)
- **Decision**: Use `tiny-skia` on CPU + `softbuffer` and stable `winit 0.30.x`. Depend only on `roughr = "0.14"` core and implement a local converter. Enforce rule: do NOT add spatial tile caching or rayon until the 5k-element benchmark demands it.
- **Status**: **Accepted (Provisional until Spike 1)** (recorded in ADR 0003).

---

### [2026-11-01] Decision 6: Text Layout via Parley & Bundled OFL Fonts
- **Decision**: Use Linebender's `parley` engine over `cosmic-text`. Bundle OFL-licensed Aref Ruqaa (headings) and Vazirmatn (body notes).
- **Context**: `cosmic-text` has open BiDi line-wrap and Arabic splitting bugs (#252, #289); `parley` uses `harfrust` (HarfBuzz) and `icu4x` for robust UAX#9 shaping.
- **Status**: **Accepted (Provisional until Spike 2)** (recorded in ADR 0007).

---

### [2026-11-01] Decision 7: UI Chrome Evaluation Strategy
- **Decision**: Keep ADR 0008 as "Proposed" evaluating `egui` vs custom CPU chrome. Do not add `egui` to the skeleton until Spikes 1 & 2 resolve GPU/CPU constraints.
- **Status**: **Proposed** (recorded in ADR 0008).

---

### [2026-11-01] Decision 8: Native Versioned JSON Format
- **Decision**: Use an independent versioned JSON document format (`.mirsam`) with root `schema_version: 1` and `format: "mirsam"` magic keys from day one. Support JSON Canvas (`.canvas`) as an export target in v0.2.
- **Status**: **Accepted** (recorded in ADR 0004).

---

### [2026-11-01] Decision 9: Kill & Pivot Gates
- **Decision**: Enforce three objective kill/pivot gates: Gate 1 (Day-14 Technical Spike Check -> ADR 0006 fallback), Gate 2 (Week-6 Dogfooding Check), and Gate 3 (Day-90 Adoption Check).
- **Status**: **Approved & Implemented** (recorded in `docs/product/kill-criteria.md`).

---

### [2026-11-01] Decision 10: AI Governance Policy
- **Decision**: Require `Assisted-by: <tool>:<model>` commit trailers for AI contributions (never Co-authored-by). Enforce strict human review and module size caps (~350 LOC).
- **Status**: **Accepted** (recorded in ADR 0009).

---

### [2026-11-01] Decision 11: English-Only Tracked Documentation
- **Decision**: All tracked repository files must be written exclusively in English. Remove `README.ar.md`. Internal founder guides in Arabic reside strictly in gitignored `.local/GUIDE.ar.md`.
- **Status**: **Approved & Implemented**.
