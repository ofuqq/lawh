# ADR 0002: Apache-2.0 Open Source License with NOTICE

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Selecting an open-source license sets the rules for how community members, researchers, and commercial organizations can use, modify, and contribute to Mirsam. We must balance protecting the project with encouraging adoption and enabling `mirsam-core` to be integrated into developer study workflows.

---

## 2. Decision Drivers
- **Ecosystem Compatibility**: Universal compatibility with Rust systems libraries and developer tools.
- **Explicit Patent Grant**: Strong legal protections for contributors and end users.
- **Low Contributor Friction**: Inbound = outbound licensing without burdensome legal agreements (no CLA/DCO for v0.1).
- **License Integrity Enforcement**: Automated license auditing in CI via `cargo-deny`.

---

## 3. Considered Options
- **Option 1**: Apache License, Version 2.0 (Permissive, patent retaliation clause, standard NOTICE).
- **Option 2**: Dual MIT / Apache-2.0 (Common in Rust libraries, but unnecessary dual complexity for an application).
- **Option 3**: GNU Affero General Public License (AGPL-3.0) (Strong copyleft, prevents closed cloud SaaS wrappers).
- **Option 4**: Proprietary / Source-Available SDK (Matches tldraw; commercial barriers and watermarks).

---

## 4. Decision Outcome & Rationale
Chosen option: **Option 1 (Apache License, Version 2.0 ONLY)**.

### Rationale
1. **Explicit Patent License (Section 3)**: Provides explicit patent grant and retaliation protections, shielding contributors and users.
2. **Standardization**: Single-license clarity eliminates ambiguity for packaging distributions and corporate study environments.
3. **Automated Enforcement**: Enforced via `cargo-deny` in CI, allowing only permissive/compatible licenses (Apache-2.0, MIT, BSD-2/3, ISC, Unicode-*, Zlib, CC0, OFL-1.1 for fonts) and denying all GPL/AGPL/LGPL copyleft licenses.

### The Explicit Trade-Off: "Fork and Close"
- Under Apache-2.0, **anyone may fork Mirsam, add proprietary modifications, and distribute closed-source binaries without contributing source code back**.
- **Considered Alternative (AGPL-3.0)**: Projects like `drafft-ink` use AGPLv3. However, Mirsam is an offline desktop client, not a networked SaaS server. AGPL creates severe corporate and academic adoption hurdles without providing meaningful defense for desktop software. Furthermore, adopting AGPL would prevent `mirsam-core` from being embedded into MIT/Apache developer tooling.
- *Mitigation*: Our defense against forks is continuous engineering speed, direct user empathy for solo learners, active community engagement, and strong brand trust under the Ofuq organization.

### Clean-Room Rule
Forking or copying code from AGPL/GPL projects (including `drafft-ink`, `rnote`, or `Xournal++`) is strictly prohibited. Mirsam is developed clean-room.
