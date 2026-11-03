# Founder Decision Log

This log records major technical, architectural, product, and governance decisions made by the repository owner.

---

### [2026-11-01] Decision 1: Rename Project from Lawh to Ruqa
- **Decision**: Adopt **Ruqa** (`ofuqq/ruqa`, crates `ruqa-core`, `ruqa-render`, `ruqa-app`) as canonical project and repository name.
- **Context**: External research report identified an active commercial company, Lawh.ai (https://lawh.ai/features), operating in the exact same Arabic/RTL vector canvas vertical. Renaming avoids trademark infringement risk (Classes 9/42) and SEO dilution. Audited candidate list: `ruqa` is clean on crates.io (404), clean on GitHub (`ofuqq/ruqa`), and has no software whiteboard collision.
- **Status**: **Approved & Implemented** (recorded in ADR 0005).

---

### [2026-11-01] Decision 2: License Switch to Apache-2.0 ONLY
- **Decision**: License Ruqa strictly under **Apache-2.0** with a `NOTICE` file; remove MIT dual-licensing. Enforce license compliance via `cargo-deny` in CI.
- **Context**: Apache-2.0 provides an explicit patent grant and strong contributor protections while remaining widely compatible across the Rust ecosystem.
- **Status**: **Accepted** (recorded in ADR 0002).

---

### [2026-11-01] Decision 3: Clean-Room Implementation & Rejection of drafft-ink Fork
- **Decision**: Build Ruqa clean-room; strictly prohibit copying code from AGPL/GPL projects (drafft-ink, Rnote, Xournal++). Reject forking drafft-ink due to AGPLv3 copyleft incompatibility with Apache-2.0.
- **Context**: Preserves license purity and developer trust.
- **Status**: **Approved & Implemented**.

---

### [2026-11-01] Decision 4: 3-Crate Architecture & Pre-Approved Tauri Fallback
- **Decision**: Scaffold greenfield 3-crate architecture (`ruqa-core`, `ruqa-render`, `ruqa-app`). Pre-approve a contingency fallback to a Tauri v2 hybrid architecture if the Day-14 Technical Spike fails.
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
- **Decision**: Use an independent versioned JSON document format (`.ruqa`) with root `schema_version: 1` and `format: "ruqa"` magic keys from day one. Support JSON Canvas (`.canvas`) as an export target in v0.2.
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

---

### [2026-11-03] Decision 12: Upstream Collaboration Protocol & Evaluation of rough_tiny_skia (#132)
- **Decision**: Endorse the architectural direction of adopting the official companion crate `rough_tiny_skia` to replace the in-tree `rough_convert.rs` and manual drawing loop once upstream lands necessary feature parity in a `0.14.x` release. Establish a formal protocol for evaluating and accepting inbound community contributions and upstream transitions:
  1. **Identity & Ownership Verification**: Confirm identity and crates.io ownership before endorsing upstream transitions (verified `orhanbalci` as owner of `roughr` and `rough_tiny_skia`).
  2. **Licensing & DCO Alignment**: Require Apache-2.0 or compatible permissive licensing without copyleft dependencies, validated by `cargo deny`. All community contributions must adhere to clean-room standards and Developer Certificate of Origin (DCO).
  3. **Technical Acceptance Criteria**: Require full feature and safety parity before replacement:
     - `draw_with_transform(&mut PixmapMut, Transform)` supporting viewport pan/zoom and HiDPI transforms.
     - Graceful, non-panicking handling of empty operation sets (`PathBuilder::finish()` returning `None`).
     - Consistent line cap and join styling across both solid and dashed stroke paths.
  4. **Regression Verification**: Require that any future migration passes the existing golden image rasterization tests in `crates/ruqa-render/tests/` without visual regressions.
- **Context**: Upstream maintainer Orhan Balci opened issue #132 proposing Ruqa transition from its internal adapter to `rough_tiny_skia`. Offloading primitive conversion to the upstream maintainer aligns with Ruqa's low-maintenance strategy while benefiting the broader ecosystem.
- **Status**: **Accepted (Blocked on upstream `rough_tiny_skia 0.14.x` release)**.

