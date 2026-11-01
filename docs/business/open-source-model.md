# Open Source Business Model & Sustainability

As a solo founder building an open-source tool, balancing community adoption, contributor friction, and long-term project sustainability is essential. This document evaluates our licensing decisions, contributor policies, sustainability models, and project risks.

---

## 1. Licensing Analysis: Permissive vs Copyleft

### The Decision: Dual MIT / Apache-2.0
Lawh is published under the dual **MIT / Apache-2.0** license, matching the canonical standard of the Rust ecosystem.

### The Explicit Trade-Off
- **Advantage (Adoption & Integration)**: Developers and companies can freely embed `lawh-core` into their tools, study workflows, and plugins without fear of licensing contamination. Permissive licensing drastically accelerates community adoption and trust.
- **The Core Risk ("Fork and Close")**: Anyone—including well-funded commercial entities—can take Lawh, add features, and redistribute it as proprietary closed-source software without giving anything back.

### The Considered Alternative: AGPLv3 (Network Copyleft)
We specifically considered the **GNU Affero General Public License (AGPL-3.0)**, as used by projects like `drafft-ink`:
- *Why AGPL is Attractive to Solo Founders*: AGPL prevents cloud providers and competitors from hosting modified versions of your code as a closed SaaS service without contributing source code back.
- *Why We Rejected AGPL for v0.1*:
  1. Lawh is a **local-first desktop tool**, not a cloud SaaS service; AGPL's network copyleft clause provides little defensive value for offline desktop binaries.
  2. AGPL creates severe adoption friction. Many enterprise developers and educational institutions are legally prohibited from installing or contributing to AGPL software.
  3. Reusability of `lawh-core` as a generic Rust library for drawing algorithms would be severely hindered.

---

## 2. Contributor Policy (Inbound = Outbound)

- **Inbound = Outbound**: All contributions submitted to Lawh are accepted under the existing dual MIT/Apache-2.0 license.
- **No CLA (Contributor License Agreement)**: CLAs create legal paperwork and discourage casual open-source contributors. We do not require a CLA.
- **No DCO (Developer Certificate of Origin) for Now**: To keep early bootstrapping frictionless, DCO sign-offs (`git commit -s`) are not enforced in v0.1. (This decision will be revisited when external contributors arrive; see [decision-log.md](decision-log.md)).

---

## 3. Trademark and Naming Protection

While the source code is permissively licensed, the **Lawh** name, logo, and identity are protected under customary trademark principles:
- Forked distributions may not claim to be the official Lawh release or use official branding without permission.
- The organization name **Ofuq** (أفق) and the project name **Lawh** (لَوْح) represent our reputation for high-quality, privacy-respecting educational software.

---

## 4. Sustainability & Monetization Options

For a solo maintainer, relying purely on unpaid passion leads to burnout. Sustainable pathways include:

1. **GitHub Sponsors & Open Collective**: Direct developer support from grateful students, educators, and power users.
2. **App Store Convenience Builds**:
   - The source code and raw Linux binaries remain 100% free on GitHub.
   - Convenient, auto-updating packaged builds can be sold for a small fee on app stores (e.g., Flathub paid tier, Steam, Microsoft Store, Mac App Store).
3. **Supporter Perks & Themes**: Providing custom cosmetic canvas themes (e.g., specialized vintage notebook or chalk themes) for financial sponsors.
4. **Educational Grants**: Applying for open-source and educational software development grants (e.g., NLnet, sovereign tech funds).

---

## 5. Risk Assessment

| Risk | Impact | Mitigation Strategy |
| :--- | :--- | :--- |
| **Big Tech / Commercial Fork** | High | Maintain faster iteration, tighter community bonds, and superior user empathy. Community loyalty follows the active maintainer, not a cold fork. |
| **Maintainer Burnout** | High | Strict MVP scope boundaries. Aggressive refusal of enterprise features (collaboration, cloud sync). Reliance on AI agents for boilerplate execution. |
| **Upstream Dependency Abandonment** | Medium | Isolate niche crates (`rough_tiny_skia`, `freedraw`) behind thin internal module wrappers. If an upstream crate stalls, we can fork and vendor it cleanly. |
| **Lack of Initial Traction** | Low | Low cost to run. Lawh solves the maintainer's own daily study needs; utility exists even with zero external users. |
