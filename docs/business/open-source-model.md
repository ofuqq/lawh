# Open Source Model & Sustainability

Ruqa is built as an open-source, local-first utility. This document outlines our licensing model, clean-room rules, sustainability options, and contributor policy.

---

## 1. Licensing Model: Apache-2.0 ONLY

Ruqa is licensed exclusively under the **Apache License, Version 2.0** with a standard `NOTICE` file.

### Why Apache-2.0?
1. **Explicit Patent License (Section 3)**: Provides strong patent retaliation protections, protecting contributors and users from patent aggression.
2. **Clear Commercial Permissions**: Companies, researchers, and students can freely use Ruqa without fear of licensing ambiguity.
3. **Ecosystem Standardization**: Aligns cleanly with modern systems libraries and tools across the Rust and developer tooling ecosystems.

### The Honest Trade-Off: "Fork and Close"
Under the Apache-2.0 license, **anyone may fork Ruqa, add proprietary features, and distribute closed-source binaries without contributing source code back**.
- **Evaluated Alternative (AGPL-3.0)**: Projects like `drafft-ink` use AGPLv3 to force source reciprocity on network hosts. However, Ruqa is an offline desktop tool, not a cloud service. Copyleft creates severe institutional friction and prevents the core geometry library (`ruqa-core`) from being reused in open-source developer tooling.
- *Our Defense*: Continuous engineering speed, direct user empathy for solo learners, active community engagement, and strong brand trust under the Ofuq organization.

---

## 2. Clean-Room Implementation Policy

Ruqa enforces a strict clean-room development standard:
- **Zero Copied Code from Copyleft Projects**: No code may be copied, vendored, or closely translated from GPL, LGPL, or AGPL codebases (such as `drafft-ink`, `flxzt/rnote`, or `Xournal++`).
- **Forking drafft-ink is Rejected**: Although drafft-ink implements useful whiteboard primitives, its AGPLv3 license makes forking legally incompatible with our Apache-2.0 decision.
- **Vendoring Rules**: Only MIT- or Apache-2.0-licensed code may be vendored, and all original copyright headers and license texts must be preserved alongside an entry in `NOTICE`.

---

## 3. Contributor Policy (Inbound = Outbound)

- **Inbound = Outbound**: All contributions submitted to Ruqa are licensed under Apache-2.0.
- **No CLA**: We do not require a Contributor License Agreement.
- **No DCO for Initial Development**: To maintain low friction for early development, Developer Certificate of Origin (`Signed-off-by`) sign-offs are not required for v0.1.
- **Revisit Trigger**: Enforcing DCO will be revisited when the first external non-founder contributor arrives (recorded in [decision-log.md](decision-log.md)).

---

## 4. Sustainability & Monetization Options

Ruqa will have **zero monetization in v0.1.0**. The software and Linux/Windows binaries are 100% free.

To sustain development long-term without crippling the core tool with paywalls, three ethical options exist for post-v0.1:

1. **Convenience App Store Builds**:
   - Source code and raw binaries on GitHub Releases remain free forever.
   - Convenient, auto-updating signed builds can be offered on app stores (e.g. Microsoft Store, Mac App Store) for a one-time fee (e.g. ~$12; revenue estimates reported in 2026 market research are unverified).
2. **Optional Hosted Study Sharing & Sync (The Obsidian/Anki Model)**:
   - For users who want one-click web publishing of lecture boards or multi-device encrypted sync without configuring Git/Syncthing, an optional hosted service (e.g. ~$3/month; estimate unverified) could be introduced in v0.3+.
3. **Public-Interest Open-Source Grants**:
   - Applying for educational and open-source infrastructure grants (e.g. NLnet Foundation in Europe, regional open-source funds).

---

## 5. Trademark & Name Protection

While the code is Apache-2.0, the name **Ruqa** and organization **Ofuq** (Ofuq) are protected under customary trademark law. Forked distributions may not use official branding without permission.
