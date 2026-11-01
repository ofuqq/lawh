# Market & Technical Research Digest: Digital Whiteboards & Study Vaults

**Date**: November 2026  
**Context**: Evaluation of external product research report (*Product Research, Market Analysis, and Technical Architecture Report: Project Lawh*, November 2026) to inform the architecture, scope, licensing, and naming of **Mirsam**.

---

## 1. Executive Summary of Research Report

The research report analyzed the 2024–2026 visual note-taking, whiteboard, and digital drawing landscape. Key insights:

1. **Market Landscape**: The market is split between heavy web/Electron collaborative canvases (Excalidraw, tldraw, Miro) and stylus-oriented Linux/tablet apps (Rnote, Xournal++, Lorien).
2. **The "Excalidraw in Rust" Territory is Occupied**: In early 2026, Patrick Wieschollek released `drafft-ink` (AGPLv3, Rust/WebGPU via vello, parley, egui, roughr), delivering collaborative sketching and LaTeX math. A pure clone of Excalidraw in Rust without study-vault ergonomics is redundant.
3. **The Unmet Need (The "Golden Gap")**: Laptop-based STEM students and self-taught developers working with a mouse and keyboard on Windows/Linux suffer from:
   - Severe web wrapper bloat (e.g., Goodnotes for Windows consuming 1.3 GB idle RAM and 1–2s stroke delay; reported by Noteastic, May 2026 [URL: https://noteastic.app/en/blog/goodnotes-slow-on-windows-heres-why], not independently verified).
   - Broken Arabic/RTL typography (Excalidraw Issues #8394, #12404, #12387 [URL: https://github.com/excalidraw/excalidraw/issues/8394]).
   - Lack of local multi-board organization and fast search across files.
4. **Brand Collision Warning**: The report identified active commercial presence of **Lawh.ai** (https://lawh.ai/features), a commercial Arabic/English vector design tool with Claude MCP integration, recommending immediate renaming to avoid trademark liability and SEO dilution.
5. **Technical Friction Points in Rust**:
   - Complex BiDi text editing on a custom canvas (cosmic-text has open BiDi line-wrap and Arabic splitting bugs #252, #289; parley backed by Linebender is more active and uses harfrust/icu4x).
   - Upstream crate version conflicts: `rough_tiny_skia 0.14` pins `tiny-skia = "0.11"`, conflicting with `tiny-skia 0.12.0`. A local 35-line converter mapping `roughr::core::Op` directly to tiny-skia paths avoids adapter crate gridlock.
   - Winit tablet pen driver instability across Windows Ink / Wintab / WM_TOUCH regressions. A strict mouse-first focus avoids this entire failure domain.

---

## 2. Research Recommendation vs. Project Decision Matrix

| Report Recommendation | Our Decision | Rationale |
| :--- | :--- | :--- |
| **Rename from Lawh** to avoid collision with Lawh.ai (Classes 9/42). | **Adopt Mirsam (مِرْسَم)** | Verified Lawh.ai conflict. Evaluated candidates: `Mirsam` is clean on crates.io, GitHub, and has zero software app collisions. |
| **Dual-license core under MIT/Apache, app under MPL-2.0 or GPL-3.0; or fork drafft-ink under AGPLv3**. | **Apache-2.0 ONLY across all crates** | Apache-2.0 provides an explicit patent grant, commercial clarity, and ecosystem alignment. Forking drafft-ink is rejected because AGPLv3 is copyleft and incompatible with Apache-2.0. |
| **Adopt Clean-Room Implementation**. | **Accepted** | Zero code copied from drafft-ink, Rnote, or GPL/AGPL tools. Architecture references only. |
| **Adopt Tauri v2 Hybrid** as immediate shortcut if shipping in 3 weeks. | **Native Rust first; Tauri v2 as pre-approved fallback (ADR 0006)** | We pursue native Rust (3 crates: `mirsam-core`, `mirsam-render`, `mirsam-app`). If the Day-14 technical spike gate fails, we trigger the pre-approved pivot to Tauri v2. |
| **Use roughr core only without adapter crates**. | **Adopted (ADR 0003)** | Avoids dependency version lock-in between `tiny-skia` and `rough_tiny_skia`. Local converter maps `roughr::core::OpSet` directly to `tiny_skia::PathBuilder`. |
| **Use parley over cosmic-text for Arabic/BiDi**. | **Adopted (ADR 0007)** | `parley` leverages Linebender’s `harfrust` (HarfBuzz port) and `icu4x`, actively addressing UAX#9 isolates and cursive shaping. |
| **Use egui for UI chrome and text editing overlay**. | **Proposed (ADR 0008)** | Evaluated in Spikes 1 & 2. egui usually requires GPU wgpu, which challenges CPU-only rendering; Spike 2 will decide between egui overlay vs parley::PlainEditor. |
| **Implement 256px spatial tile cache + rayon immediately**. | **Provisional Rule**: Do not add tile cache or rayon until benchmark proves necessity | Avoid premature optimization. Test single-threaded tiny-skia first; introduce tile caching only if the 5k-element benchmark fails. |
| **Adopt JSON Canvas (.canvas) as native format**. | **Use native versioned JSON (.mirsam); JSON Canvas as v0.2 export** | Native format requires magic key, schema versioning, and freehand/rough style parameters that JSON Canvas 1.0 cannot express. |
| **Add Active-Recall Study Tape & Numbered Frames**. | **Scheduled for v0.2 / v0.3** | Excellent for student personas, but scoped after core drawing, selection, vault persistence, and Arabic text are solid. |
| **Adopt Assisted-by commit trailers for AI code**. | **Adopted (ADR 0009)** | Standardizes human accountability: `Assisted-by: <tool>:<model>`, mandatory human review, no unreviewed dependencies. |

---

## 3. Verified vs. Unverified Research Claims

To uphold our strict rule against inventing or propagating unverified facts, all external claims from the research report are categorized below:

### Independently Verified by Us
- **crates.io availability**: Verified that `mirsam`, `mirsam-core`, `saboora`, `saboora-core`, `karrasa`, and `karrasa-core` were unregistered (HTTP 404).
- **Lawh.ai collision**: Confirmed live commercial operation of Lawh.ai offering Arabic/RTL vector design and Claude MCP integration (https://lawh.ai/features).
- **rough_tiny_skia dependency pinning**: Confirmed that `rough_tiny_skia 0.14.0` depends on `tiny-skia = "^0.11"`, conflicting with `tiny-skia 0.12.0`.
- **drafft-ink license & stack**: Confirmed `PatWie/drafft-ink` is AGPL-3.0 using vello, parley, egui, and roughr.
- **GitHub Actions pinned SHAs**: Verified `actions/checkout@v4.2.2`, `dtolnay/rust-toolchain`, and `Swatinem/rust-cache` commit hashes against GitHub tags.
- **Rulesets schema**: Verified against GitHub REST API (`POST /repos/ofuqq/mirsam/rulesets`).

### Unverified Claims from the Research Report (Reported by Source, Not Independently Verified)
1. **Goodnotes Windows RAM & Latency**: Reported by Noteastic Blog (May 2026) [https://noteastic.app/en/blog/goodnotes-slow-on-windows-heres-why] as consuming 1.3 GB idle RAM and 1–2s pen delay on Windows. *Not independently verified on our hardware.*
2. **Freeform Overheating & Data Loss**: Reported in Reddit threads (r/ipad, 2024–2026) [https://www.reddit.com/r/ipad/comments/1ruk5xl/freeform_becomes_unusable_with_large_boards_ipad/]. *Not independently verified.*
3. **drafft-ink Long-Term Multi-User Stability**: Reported 570 stars and 74 forks in August 2026 [https://github.com/PatWie/drafft-ink]. *Desktop long-term stability on large multi-thousand stroke study boards remains unverified.*
4. **Desigma Benchmark Numbers**: Reported by Indie Machine (2026) [https://indie-machine.com/posts/desigma-rust-native-design-tool-review] as achieving <4ms pan time on 5,000 layers with 256px tile cache. *Treated strictly as a performance hypothesis to test in Spike 1.*
5. **winit Tablet Pen Regressions**: Reported in winit issues #99, #4493, #4591 [https://github.com/rust-windowing/winit/issues/99] regarding Windows Ink WM_TOUCH overrides. *Validates our decision to remain mouse-first in v0.1.*

---

## 4. Key Source Index

- [1] `PatWie/drafft-ink`: https://github.com/PatWie/drafft-ink (AGPL-3.0, Rust, vello, parley, egui, roughr, 2026)
- [4] Excalidraw Issue #8394 ("Suggestion: Add a font for Arabic/Persian"): https://github.com/excalidraw/excalidraw/issues/8394
- [5] Excalidraw Issue #7280 ("Performance degraded with large drawings (5000+)"): https://github.com/excalidraw/excalidraw/issues/7280
- [6] Noteastic Blog ("Why Goodnotes is Slow on Windows"): https://noteastic.app/en/blog/goodnotes-slow-on-windows-heres-why
- [9] `flxzt/rnote`: https://github.com/flxzt/rnote (GPL-3.0, GTK4/Cairo stylus notes app, 2026)
- [10] Lawh.ai: https://lawh.ai/features (Commercial Arabic/RTL design canvas, 2026)
- [12] `pop-os/cosmic-text` Issue #252 ("BiDi text overflows buffer instead of wrapping"): https://github.com/pop-os/cosmic-text/issues/252
- [13] `linebender/parley` Issue #25 ("Support inline box layout and BiDi LRI/PDI"): https://github.com/linebender/parley/issues/25
- [14] `orhanbalci/rough-rs`: https://github.com/orhanbalci/rough-rs (MIT, pure-Rust Rough.js port, roughr 0.14.0)
- [15] `ducflair/freedraw`: https://github.com/ducflair/freedraw (MIT, Rust port of perfect-freehand, 1.0.4)
- [30] JSON Canvas 1.0 Specification: https://jsoncanvas.org/spec/1.0/
- [67] Linebender Blog #25: https://linebender.org/blog/tmil-25/ (parley, vello, sparse_strips, 2026)
- [102] SSD Nodes ("Where Open Source Policies on AI Code Have Landed"): https://www.ssdnodes.com/learn/ai-assisted-code-open-source-policies
