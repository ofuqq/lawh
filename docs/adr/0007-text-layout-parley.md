# ADR 0007: Text Layout & Shaping Engine (Parley)

- **Status**: **Accepted** (Provisional until Spike 2)
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Mirsam requires text rendering on an infinite canvas. While text is secondary in early MVP milestone M1, bilingual Arabic/English text is a core product pillar planned for Milestone M2/M5.
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

We select **Option 1: `parley`** as our primary text layout and shaping engine, provisional until verified in **Spike 2**.

### Implementation Strategy:
1. **Font Strategy**: Bundle an open-source, permissive font under the SIL Open Font License (OFL-1.1), such as `Cairo` or `Amiri` for Arabic and `Inter` for Latin/math numerals.
2. **Spike 2 Verification Criteria**:
   - Render a mixed sentence: "Hello عالم (World) 123!".
   - Correctly shape Arabic cursive forms without glyph detachment.
   - Accurately calculate glyph bounding boxes for cursor hit-testing and text selection.
   - Measure shaping latency (< 5 ms for 500 characters).

If Spike 2 reveals severe API instability or integration blockers with our `tiny-skia` renderer, the team will evaluate fallback options or trigger ADR 0006.
