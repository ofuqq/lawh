# Typography & Bidirectional (Arabic/RTL) Text Architecture

> **Status: Design Proposal (Draft)**  
> This document specifies the text shaping, bidirectional layout, and font bundling architecture in `ruqa-render`.

---

## 1. Engine Selection: Why `parley` Over `cosmic-text`?

High-quality Arabic typography requires five distinct technical capabilities working in unison:
1. **Cursive Contextual Shaping**: Letters change form (isolated, initial, medial, final) and form mandatory ligatures (e.g., لا) based on context.
2. **Bidirectional Reordering (Unicode UAX#9)**: Mixing RTL Arabic and LTR Latin/numbers in the same paragraph without reversing punctuation or brackets.
3. **Strict LTR Isolates**: Technical tokens ($O(V \log V)$, `#ffffff`, `Ctrl+X`, `code`) must remain left-to-right inside an Arabic sentence.
4. **Automatic Font Fallback**: Seamlessly switching from Latin font to Arabic font when Arabic characters appear.
5. **Caret & Selection Mapping**: Translating logical byte offsets into visual cursor positions across split BiDi runs.

### Evaluation of Candidate Engines
- **`cosmic-text` (Rejected)**: Sits under active development by Pop!_OS/System76, but has suffered persistent, open Arabic/BiDi bugs throughout 2024–2026:
  - Line-wrap buffer overflows with BiDi text (Issue #252).
  - Mid-word Arabic letter splitting across line breaks (Issue #289).
  - Punctuation and neutral symbol misplacement in mixed strings (Issue #512).
- **Linebender's `parley` (Adopted in ADR 0007)**:
  - Developed by the Linebender collective and Google Fonts team.
  - Built on `harfrust` (pure-Rust port of HarfBuzz) and `icu4x` for robust Unicode bidirectional analysis.
  - Actively fixes BiDi edge cases (PR #907 resetting trailing whitespace to paragraph BiDi level, October 2026).
  - Powers text in `drafft-ink` and other modern Rust graphics projects.

---

## 2. Bundled Open-Source Fonts

To guarantee consistent, beautiful typography on all platforms without relying on missing system fonts:

| Font Family | Style / Purpose | License | Source / Upstream |
| :--- | :--- | :--- | :--- |
| **Aref Ruqaa** | Display, headings, rough titles | **OFL-1.1** | Google Fonts / Khaled Hosny |
| **Vazirmatn** | Crisp body text, diagram notes | **OFL-1.1** | Google Fonts / Saber Rastikerdar |
| **Inter / Roboto** | Clean Latin fallback & numbers | **OFL-1.1 / Apache-2.0** | Google Fonts |

Both font files will be embedded directly in the binary or bundled in an `assets/fonts/` directory, licensed under the Open Font License (OFL-1.1).

---

## 3. Handling Mixed Arabic and Technical Tokens (UAX#9)

When a sentence contains both Arabic and English technical terms:
```text
خوارزمية Dijkstra تعمل في O(V log V)
```
Unicode treats parentheses and spaces as bidirectional neutral characters. Without explicit isolation, the parentheses will flip and attach to the wrong side of the English token.

Ruqa wraps embedded code, math, and shortcut tokens in explicit **Unicode Left-to-Right Isolates**:
- Prefix: `U+2066` (`LEFT-TO-RIGHT ISOLATE` / `LRI`)
- Suffix: `U+2069` (`POP DIRECTIONAL ISOLATE` / `PDI`)

This guarantees that English expressions render strictly left-to-right within right-to-left sentences without altering the parent paragraph direction.

---

## 4. Staged Text Editing Plan

1. **Stage 1 (v0.1.0 MVP)**:
   - Parley renders static text nodes into `tiny-skia` surfaces with automatic first-strong-character direction detection.
   - On double-click, a focused text-input buffer opens (evaluated in Spike 2: lightweight `egui::TextEdit` overlay vs `parley::PlainEditor` direct buffer).
2. **Stage 2 (v0.2.0)**:
   - Bound text auto-grow inside shapes.
   - Direct canvas caret click-and-drag hit-testing.
   - Automatic inline backtick wrapping (`code`) into LTR isolates.
3. **Stage 3 (v0.3.0+)**:
   - Full Arabic UI localization with mirrored toolbar layout.
