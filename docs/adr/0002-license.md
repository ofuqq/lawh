# ADR 0002: Dual MIT / Apache-2.0 Open Source License

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Selecting an open-source license sets the rules for how community members, educational institutions, and commercial organizations can use, modify, and contribute to Lawh. We must balance protecting the project with encouraging adoption and enabling `lawh-core` to be widely integrated into the Rust ecosystem.

---

## 2. Decision Drivers
- **Rust Ecosystem Compatibility**: Adherence to standard conventions across crates.io.
- **Maximum Adoption**: Low friction for students, educators, developers, and institutions.
- **Contributor Simplicity**: Inbound = outbound licensing without burdensome legal paperwork.
- **Defensive Safeguards**: Patent grant protection and clear contributor liability disclaimers.

---

## 3. Considered Options
- **Option 1**: Dual MIT OR Apache-2.0 (Permissive, Rust ecosystem standard).
- **Option 2**: GNU Affero General Public License (AGPL-3.0) (Strong copyleft, prevents proprietary SaaS exploitation).
- **Option 3**: MIT-Only (Ultra-simple permissive, matches Excalidraw).
- **Option 4**: Business Source License / Custom SDK License (Matches tldraw, commercial restrictions).

---

## 4. Decision Outcome & Rationale
Chosen option: **Option 1 (Dual MIT OR Apache-2.0)**.

### Rationale
1. **Rust Ecosystem Alignment**: Almost all foundational Rust crates (`serde`, `winit`, `tokio`, `vello`, `cosmic-text`) use dual MIT/Apache-2.0. This allows `lawh-core` and future Lawh libraries to be consumed anywhere without license conflicts.
2. **Apache 2.0 Patent Grant**: While MIT is short and permissive, Apache 2.0 provides an explicit patent retaliation clause (Section 3), protecting contributors and users from patent aggression.
3. **Low Barrier to Entry**: Permissive licenses encourage developers to embed Lawh into their tools, study workflows, and plugins.

### The Explicit Trade-Off: "Fork and Close"
- Under a permissive license, **anyone can fork Lawh, modify the code, and distribute it as closed-source proprietary software** without contributing modifications back.
- **Considered Alternative (AGPLv3)**: We evaluated AGPLv3 (used by `drafft-ink`) to prevent closed-source forks. However, Lawh is an offline desktop client, not a networked SaaS backend. AGPL creates severe corporate and academic adoption hurdles without providing meaningful defense for desktop software.
- *Mitigation*: Our defense against forks is continuous innovation, strong brand reputation (Ofuq / Lawh), deep user empathy for solo learners, and active community engagement.
