# ADR 0007: Text Layout & Shaping Engine (Parley)

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Ruqa requires text rendering on an infinite canvas. While text is secondary in early MVP milestone M1, bilingual Arabic/English text is a core product pillar planned for Milestone M2/M5.
Arabic text layout presents strict requirements:
1. **Right-to-Left (RTL) & BiDi**: Complex mixed directional runs (Unicode Bidirectional Algorithm, UAX #9).
2. **Cursive Font Shaping**: Contextual glyph shaping (initial, medial, final, isolated forms) and ligature substitution.
3. **Line Breaking & Justification**: Kashida (tatweel) or proportional spacing without breaking cursive runs.

---

## 2. Considered Options

### Option 1: `parley` (Linebender Ecosystem)
- Built on `harfrust` (pure Rust HarfBuzz text shaping port) and `icu4x` (Unicode internationalization components from Unicode Consortium).
- Designed specifically for advanced internationalized typography and robust BiDi support.
- Backed by the Linebender community with rigorous Unicode compliance focus.

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
   - `ArefRuqaa-Regular.ttf` (SIL Open Font License OFL-1.1) for Arabic calligraphy.
   - `Roboto-Regular.ttf` (Apache-2.0) for Latin/math numerals.
2. **Validation Evidence (Spike 2 Benchmark)**:
   On 2026-11-01, Spike 2 ([report](../research/spikes/spike-2-arabic-shaping.md)) validated shaping on the test string `خوارزمية Dijkstra تعمل في O(V log V)` and a 440-character technical passage:
   - Arabic cursive connected cleanly with zero detached glyphs.
   - Latin tokens (`Dijkstra`, `O(V log V)`) remained strictly LTR inside the RTL sentence.
   - Mean shaping latency across 100 iterations was **$1.77\text{ ms}$** (Min: $1.06\text{ ms}$, Max: $4.18\text{ ms}$), easily beating the $< 5.0\text{ ms}$ requirement.
   - All transitive dependencies passed `cargo-deny` with zero license conflicts.
