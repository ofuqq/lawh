# ADR 0005: Project Naming Lineage & Selection of Ruqa

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
The project has evolved through three naming phases before establishing its permanent public identity:

1. **Lawh (Initial Bootstrap Working Name)**:
   - Meaning: Arabic term for slate, tablet, or whiteboard.
   - Collision: Market research on 2026-11-01 identified active venture-backed commercial operation of **Lawh.ai** (`https://lawh.ai/features`), a proprietary AI design editor with first-class Arabic/RTL vector typography. Retaining "Lawh" posed immediate trademark liability (International Classes 9 & 42) and severe SEO dilution.
2. **Mirsam (Interim Working Name)**:
   - Meaning: Arabic term for easel, art studio, or drafting stylus.
   - Evaluated as clean on registries, but replaced by the founder in favor of a name more deeply evocative of handwriting, speed, and study notes.
3. **Ruqa (Final Chosen Name)**:
   - Etymology: Derived from **Ruq'ah**, the renowned Arabic calligraphic and handwriting style developed for rapid, everyday note-taking, clarity, and practical study without superfluous ornamental complexity.
   - Conveys lightweight efficiency, personal study, clean strokes, and authentic cultural heritage.

---

## 2. Name Search & Registry Audit Evidence

Verification details for **Ruqa** (audited 2026-11-01):
- **crates.io Registry**:
  - `ruqa`: **Available** (HTTP 404 verified).
  - `ruqa-core`: **Available** (HTTP 404 verified).
  - `ruqa-render`: **Available** (HTTP 404 verified).
  - `ruqa-app`: **Available** (HTTP 404 verified).
- **GitHub Namespace**:
  - `ofuqq/ruqa`: Available and successfully claimed.
  - Repository renamed from `ofuqq/mirsam` to `ofuqq/ruqa`.
  - Remote URLs updated to `https://github.com/ofuqq/ruqa.git`.
- **Global Software Search**:
  - Checked web and open-source registries for conflicting software.
  - Unrelated projects exist (e.g. Ruqola, a KDE chat client; wedding photography studios; Quranic ruqyah tools), but zero competing whiteboard, drawing, canvas, or study note products.

---

## 3. Decision Outcome & Implementation

The project is permanently named **Ruqa**.

### Actions Taken:
1. **GitHub Repository**: Canonical location is [`ofuqq/ruqa`](https://github.com/ofuqq/ruqa).
2. **Crates Namespace**:
   - `crates/ruqa-core`: Core geometry, scene graph, undo/redo, and file I/O.
   - `crates/ruqa-render`: 2D rasterization, roughr converter, softbuffer presentation, and text layout.
   - `crates/ruqa-app`: Native OS windowing, event loop, and user chrome.
3. **File Format**: Extension is `.ruqa` (UTF-8 JSON with `"format": "ruqa"` top-level identifier).
4. **Documentation**: All architecture, product, governance, and onboarding files updated to Ruqa.
5. **Language Policy**: Repository files remain strictly English-only, transliterating the calligraphic style as "Ruq'ah" without Arabic script.
