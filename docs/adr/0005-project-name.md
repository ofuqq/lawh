# ADR 0005: Project Naming & Crates Ecosystem Namespace

- **Status**: **Proposed**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Naming an open-source software project requires checking package registries (crates.io), code hosting platforms (GitHub), and domain namespaces to prevent namespace collisions, squatting conflicts, and trademark ambiguities.

---

## 2. Registry & Platform Audit Findings

1. **crates.io Registry Verification**:
   - `lawh`: **Available** (crate does not exist on crates.io).
   - `lawh-core`: **Available** (crate does not exist on crates.io).
2. **GitHub Namespace**:
   - `afaq/lawh` does not exist; the top-level user account `Afaq` is an inactive personal account registered in 2012.
   - The founder’s verified organization is **`ofuqq`** (Ofuq / أفق).
   - The repository **`ofuqq/lawh`** is already initialized and owned by the founder.
   - Global GitHub search shows no competing open-source whiteboard under the exact name `lawh` (the closest related tool is `MarwanAlghamdi/Lawha`, an Excalidraw wrapper in TS/React).

---

## 3. Considered Options
- **Option 1**: Keep **Lawh** (`ofuqq/lawh`, crate `lawh-core`, app `lawh`)
  - Arabic root: لَوْح (meaning slate, tablet, or board).
  - Short, distinctive, 4 characters, completely free on crates.io.
- **Option 2**: **Lawha** (`ofuqq/lawha`)
  - Feminine form (لوحة, painting/canvas).
  - Collides with existing `Lawha` repository on GitHub.
- **Option 3**: **OfuqDraw** (`ofuqq/ofuq-draw`)
  - Brand-centric name, but generic.

---

## 4. Decision Outcome & Rationale
Proposed option: **Option 1 (Lawh)**.

### Rationale
- **Cultural & Linguistic Identity**: Directly reflects the mission of an approachable digital slate/board while honoring Arabic linguistic roots.
- **Namespace Cleanliness**: Both `lawh` and `lawh-core` are 100% available on crates.io, giving our project an uncluttered library and application package namespace.
- **Conciseness**: Easy to type in terminal commands (`cargo install lawh`, `lawh notes.lawh`).
