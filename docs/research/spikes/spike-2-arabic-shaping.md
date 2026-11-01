# Spike 2: Arabic & Bilingual Text Shaping with Parley

- **Date**: 2026-11-01
- **Status**: Completed (Validation Successful)
- **Associated ADR**: [ADR 0007: Text Layout & Shaping Engine (Parley)](../../adr/0007-text-layout-parley.md)
- **Associated Issue**: [#4: Spike: Prototype Arabic and bilingual text shaping with Parley](https://github.com/ofuqq/ruqa/issues/4)

---

## 1. Objective

Prove whether Linebender's `parley` (backed by `harfrust` pure-Rust HarfBuzz port and `icu4x`) satisfies Ruqa's bidirectional and cursive text requirements for bilingual study notes, and measure paragraph shaping latency against our $< 5\text{ ms}$ budget.

### Key Verification Criteria:
1. **Contextual Arabic Cursive Shaping**: Letters connect correctly in initial, medial, and final forms without rogue mid-word gaps.
2. **Strict LTR Isolation**: Latin math, variables, and code tokens (e.g. `O(V log V)`) remain strictly left-to-right inside right-to-left Arabic sentences.
3. **Cluster Bounding & Caret Navigation**: Glyph clusters yield accurate visual offsets and advances for cursor positioning and text selection.
4. **Shaping Performance**: Shaping latency for a standard 400–500 character bilingual paragraph remains $< 5\text{ ms}$.

---

## 2. Test Environment & Bundled Fonts

- **CPU**: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz (8 threads)
- **OS**: Linux 7.2.11-arch1-1 x86_64
- **Rust Toolchain**: `rustc 1.91.0`, Cargo profile: `release` (opt-level = 3)
- **Bundled Fonts**:
  - `ArefRuqaa-Regular.ttf` (Khaled Hosny, OFL-1.1): Arabic primary calligraphy font
  - `Roboto-Regular.ttf` (Christian Robertson / Google, Apache-2.0): Latin fallback & numerals

---

## 3. Bilingual Test String Results

### Target String
$$\text{"خوارزمية Dijkstra تعمل في O(V log V)"}$$

### Layout Dimensions & Run Analysis
- **Formatted Dimensions**: $333.55 \times 37.00\text{ px}$ (at font size 24.0px)
- **Total Clusters**: 36 distinct glyph clusters

| Run Index | Text Slice | Direction | Glyph Count | Advance Width | Notes |
|:---:|:---|:---:|:---:|:---:|:---|
| **0** | `O(V log V)` | **LTR** | 10 | 112.37 px | Latin math token preserved strictly LTR |
| **1** | `تعمل في ` | **RTL** | 11 | 55.73 px | Arabic cursive shaped (medial/final connected) |
| **2** | ` ` | **RTL** | 1 | 7.99 px | Directional word spacing |
| **3** | `Dijkstra` | **LTR** | 8 | 86.28 px | Proper noun preserved strictly LTR |
| **4** | `خوارزمية ` | **RTL** | 13 | 71.18 px | Arabic initial, medial, ligature connected |

### Qualitative Verification:
- **Arabic Cursive Connected**: `true`. All Arabic words connected without glyph detachment.
- **Latin Tokens Isolated**: `true`. No directional reversal of brackets or Latin characters.
- **Cluster Hit-Testing**: `true`. Visual offsets and cluster bounds accurately resolve each character.

---

## 4. Latency Benchmark (500-Character Bilingual Paragraph)

We measured 100 iterations of full layout generation on a 440-character mixed Arabic/English technical passage discussing Dijkstra's algorithm, time complexity ($O(E + V \log V)$), and routing protocols (OSPF, IS-IS):

| Metric | Measured Value | Budget Target | Status |
|:---|:---:|:---:|:---:|
| **Minimum Latency** | **1.057 ms** | — | — |
| **Mean Latency** | **1.768 ms** | $< 5.0\text{ ms}$ | **PASSED** |
| **Maximum Latency** | **4.184 ms** | $< 5.0\text{ ms}$ | **PASSED** |

At **$1.77\text{ ms}$ average latency**, Parley easily satisfies the 60 FPS frame time budget ($< 16.6\text{ ms}$) even during continuous typing.

---

## 5. Architectural Conclusions & ADR 0007 Status

1. **ADR 0007 Accepted**:
   `parley` (version `0.12.x`) is confirmed as the text layout engine for Ruqa. It completely avoids the open bugs observed in `cosmic-text` (BiDi wrap overflow #252 and mid-word cursive disconnect #289).
2. **Zero Incompatible Licenses**:
   All dependencies in the `parley` 0.12 dependency tree comply with Apache-2.0 / MIT and pass `cargo-deny` with zero violations.
3. **Offline Font Bundling**:
   Bundling `Aref Ruqaa` and `Roboto` under `assets/fonts/` ensures zero system font dependencies and 100% reproducible rendering in headless CI runners.
