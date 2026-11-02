# ADR 0007: Text Layout & Shaping Engine (Parley)

- **Status**: **Accepted**
- **Date**: 2026-11-01 (Updated 2026-11-02)
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Ruqa requires text rendering on an infinite canvas. Bilingual Arabic/English text is Ruqa's primary competitive differentiator and core product pillar for Milestone M4 (v0.1.0) and M6 (v0.2.0).

Arabic text layout presents strict requirements:
1. **Right-to-Left (RTL) & BiDi**: Complex mixed directional runs (Unicode Bidirectional Algorithm, UAX #9).
2. **Cursive Font Shaping**: Contextual glyph shaping (initial, medial, final, isolated forms) and ligature substitution.
3. **Line Breaking & Justification**: Proportional spacing without breaking cursive runs.

---

## 2. Considered Options

### Option 1: `parley` (Linebender Ecosystem)
- Built on `harfrust` (pure Rust HarfBuzz text shaping port) and `icu4x` (Unicode internationalization components from Unicode Consortium).
- Designed specifically for advanced internationalized typography and robust BiDi support.
- Backed by the Linebender community with rigorous Unicode compliance focus.
- Pure Rust, zero system C library dependencies.

### Option 2: `cosmic-text` (Pop!_OS / System76)
- Comprehensive multi-backend text layout engine used widely in the iced/cosmic desktop ecosystem.
- Known issues: The 2026-11 technical research report and issue tracker audit identified recurring BiDi/Arabic edge cases:
  - Issue #252: BiDi line-wrapping overflow in mixed RTL/LTR blocks.
  - Issue #289: Erroneous mid-word glyph disconnects in Arabic cursive shaping under tight bounding boxes.
  - Issue #512: Directional punctuation and neutral symbol reordering anomalies.

---

## 3. Decision Outcome

We select **Option 1: `parley`** (`0.12.x`) as our primary text layout and shaping engine.

### Implementation Strategy:
1. **Font Strategy**: Bundle permissive fonts under `assets/fonts/`:
   - `ArefRuqaa-Regular.ttf` (SIL Open Font License OFL-1.1) for Arabic handwriting.
   - `Roboto-Regular.ttf` (Apache-2.0) for Latin/math numerals.
   - *Note*: Bug [#92](https://github.com/ofuqq/ruqa/issues/92) tracks replacing an initial corrupted HTML download of `Roboto-Regular.ttf` with the verified binary TrueType font.
2. **License Compliance**: Added `assets/fonts/OFL.txt` and `assets/fonts/LICENSE.apache` and attributed both fonts in `NOTICE`.
3. **Validation Evidence (Spike 2 Benchmark)**:
   Empirical benchmarking was executed via `cargo run --release -p ruqa-render --bin spike2_bench` on an Intel Core i5-8250U CPU across 100 shaping passes of a 440-character technical text:
   - **Tested String**: `"خوارزمية Dijkstra تعمل في O(V log V)"` + 440-char bilingual algorithm explanation.
   - **Arabic Cursive Connected**: `true` (zero disjoint letter glyphs).
   - **Latin Tokens Strictly Isolated**: `true` (`Dijkstra` and `O(V log V)` remained strictly LTR inside RTL sentence).
   - **Shaping Latency**:
     - Mean: **$0.886\text{ ms}$**
     - Minimum: **$0.680\text{ ms}$**
     - Maximum: **$1.611\text{ ms}$**
   - **Latency Budget**: $< 5.0\text{ ms}$ (beaten by $5.6\times$).
   - All transitive dependencies passed `cargo-deny` with zero license conflicts.
