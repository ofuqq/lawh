# ADR 0005: Project Naming & Renaming to Mirsam

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
The project was originally bootstrapped under the working name **Lawh** (Arabic: لَوْح, meaning slate, whiteboard, or tablet).
During post-bootstrap market research on 2026-11-01, an active commercial conflict was discovered:
**Lawh.ai** (`https://lawh.ai/features`), a proprietary, venture-backed AI design and graphic editor with explicit first-class Arabic and RTL typography support.
Although `lawh` was technically unallocated on crates.io, retaining "Lawh" for an open-source whiteboard with planned Arabic text support invites unavoidable brand confusion, SEO cannibalization, and trademark risks.

Therefore, the project required renaming before any public release.

---

## 2. Name Search & Registry Audit

The deciders evaluated three primary candidate names (Arabic study/canvas roots):

| Candidate | Meaning | GitHub Namespace (`ofuqq/<name>`) | crates.io (`<name>`) | crates.io (`<name>-core`) | External Conflict Check | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Mirsam** (مِرْسَم) | Easel, art/drawing studio, drafting stylus | Available (`ofuqq/mirsam`) | **Available** (404 verified) | **Available** (404 verified) | No competing commercial whiteboard or design software | **Selected (Clean)** |
| **Saboora** (سَبُّورة) | Blackboard / Chalkboard | Available | Available | Available | Not evaluated further; secondary candidate | Alternative |
| **Karrasa** (كُرَّاسَة) | Notebook / Exercise booklet | Available | Available | Available | Not evaluated further; tertiary candidate | Alternative |

Verification details for **Mirsam** (checked 2026-11-01):
- crates.io API check `https://crates.io/api/v1/crates/mirsam`: returned HTTP 404 (Unallocated).
- crates.io API check `https://crates.io/api/v1/crates/mirsam-core`: returned HTTP 404 (Unallocated).
- GitHub repository renamed from `ofuqq/lawh` to `ofuqq/mirsam`.
- Web search: no major software tool or SaaS product operating under "Mirsam Whiteboard".

---

## 3. Decision Outcome & Implementation

The project is officially renamed **Mirsam**.

### Actions Taken:
1. **GitHub Repository**: Renamed via GitHub CLI to `ofuqq/mirsam`. Remote URLs updated accordingly.
2. **Crate Manifests**:
   - `crates/lawh-core` renamed to `crates/mirsam-core`.
   - Planned renderer crate: `mirsam-render`.
   - Planned application crate: `mirsam-app`.
3. **File Format**: Canonical file extension remains `.mirsam` (JSON schema `mirsam-whiteboard-v1`).
4. **Documentation**: All architecture, product, business, and onboarding documents updated to reference Mirsam.
5. **Historical Note**: Prior references to "Lawh" in early bootstrap commit logs or ADR 0005 are retained solely for lineage tracking.
