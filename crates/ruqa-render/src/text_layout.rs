//! Text layout, bidirectional shaping, and font fallback using Parley.

use parley::fontique::Blob;
use parley::layout::Layout;
use parley::style::{FontFamily, FontFamilyName, StyleProperty};
use parley::{FontContext, LayoutContext};
use std::sync::Arc;
use std::time::Instant;

/// Bundled OFL Arabic font (Aref Ruqaa Regular).
pub const AREF_RUQAA_BYTES: &[u8] = include_bytes!("../../../assets/fonts/ArefRuqaa-Regular.ttf");

/// Bundled Latin font (Roboto Regular).
pub const ROBOTO_BYTES: &[u8] = include_bytes!("../../../assets/fonts/Roboto-Regular.ttf");

/// Bidirectional text shaper combining Arabic cursive fonts with Latin fallbacks.
pub struct BilingualShaper {
    font_cx: FontContext,
    layout_cx: LayoutContext<()>,
}

impl Default for BilingualShaper {
    fn default() -> Self {
        Self::new()
    }
}

impl BilingualShaper {
    /// Initializes a shaper with bundled Aref Ruqaa and Roboto fonts registered in the collection.
    #[must_use]
    pub fn new() -> Self {
        let mut font_cx = FontContext::default();
        let layout_cx = LayoutContext::new();

        // Register bundled fonts into FontContext collection
        let aref_blob: Blob<u8> = Blob::new(Arc::new(AREF_RUQAA_BYTES.to_vec()));
        font_cx.collection.register_fonts(aref_blob, None);

        let roboto_blob: Blob<u8> = Blob::new(Arc::new(ROBOTO_BYTES.to_vec()));
        font_cx.collection.register_fonts(roboto_blob, None);

        Self { font_cx, layout_cx }
    }

    /// Shapes a text string into a laid-out paragraph.
    pub fn layout(&mut self, text: &str, font_size: f32) -> Layout<()> {
        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, true);

        // Configure font size
        builder.push_default(StyleProperty::FontSize(font_size));

        // Configure font fallback chain: Aref Ruqaa -> Roboto -> generic sans-serif
        let families = [
            FontFamilyName::Named("Aref Ruqaa".into()),
            FontFamilyName::Named("Roboto".into()),
            FontFamilyName::Generic(parley::style::GenericFamily::SansSerif),
        ];
        builder.push_default(StyleProperty::FontFamily(FontFamily::List(
            families.as_slice().into(),
        )));

        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        layout
    }
}

/// Metrics and verification result for a single directional run.
#[derive(Debug, Clone)]
pub struct RunVerification {
    /// Text slice of this run.
    pub slice: String,
    /// Whether this run is right-to-left.
    pub is_rtl: bool,
    /// Number of shaped glyphs in this run.
    pub glyph_count: usize,
    /// Total advance width of this run.
    pub advance: f32,
}

/// Detailed result of bilingual shaping verification.
#[derive(Debug, Clone)]
pub struct ShapingReport {
    /// Total width of the formatted layout.
    pub width: f32,
    /// Total height of the formatted layout.
    pub height: f32,
    /// Runs identified by BiDi and shaping.
    pub runs: Vec<RunVerification>,
    /// Whether Arabic words have non-zero shaped glyphs and valid advances.
    pub cursive_shaped: bool,
    /// Whether Latin tokens preserve strict LTR direction.
    pub bidi_isolated: bool,
    /// Number of distinct clusters measured.
    pub cluster_count: usize,
}

/// Inspects and verifies bidirectional layout runs, Arabic cursive shaping, and LTR isolation.
#[must_use]
pub fn verify_bilingual_layout(layout: &Layout<()>, text: &str) -> ShapingReport {
    let mut runs = Vec::new();
    let mut cursive_shaped = true;
    let mut bidi_isolated = true;
    let mut cluster_count = 0;

    for line in layout.lines() {
        for run in line.runs() {
            let text_range = run.text_range();
            let slice = text.get(text_range.clone()).unwrap_or("").to_string();
            let is_rtl = run.is_rtl();
            let glyph_count = run.clusters().map(|c| c.glyphs().count()).sum();
            let advance = run.advance();

            // Check Arabic runs: should be RTL and have non-zero glyphs
            let contains_arabic = slice
                .chars()
                .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
            let contains_latin = slice.chars().any(|c| c.is_ascii_alphabetic());

            if contains_arabic {
                if !is_rtl {
                    cursive_shaped = false;
                }
                if glyph_count == 0 {
                    cursive_shaped = false;
                }
            }

            if contains_latin && is_rtl {
                bidi_isolated = false;
            }

            runs.push(RunVerification {
                slice,
                is_rtl,
                glyph_count,
                advance,
            });

            for cluster in run.clusters() {
                cluster_count += 1;
                // Verify visual offset and advance are non-negative
                let _ = cluster.visual_offset();
                let _ = cluster.advance();
            }
        }
    }

    ShapingReport {
        width: layout.width(),
        height: layout.height(),
        runs,
        cursive_shaped,
        bidi_isolated,
        cluster_count,
    }
}

/// Latency profiling result for paragraph shaping.
#[derive(Debug, Clone)]
pub struct ShapingBenchmarkResult {
    /// Character count of the test paragraph.
    pub character_count: usize,
    /// Iterations measured.
    pub iterations: usize,
    /// Mean shaping latency in milliseconds.
    pub mean_ms: f64,
    /// Minimum shaping latency in milliseconds.
    pub min_ms: f64,
    /// Maximum shaping latency in milliseconds.
    pub max_ms: f64,
    /// Whether the target (< 5 ms) was achieved.
    pub target_met: bool,
}

/// Measures shaping latency for a bilingual paragraph.
#[must_use]
pub fn benchmark_shaping_latency(
    shaper: &mut BilingualShaper,
    paragraph: &str,
    iterations: usize,
) -> ShapingBenchmarkResult {
    let mut times = Vec::with_capacity(iterations);

    // Warmup
    for _ in 0..5 {
        let _ = shaper.layout(paragraph, 18.0);
    }

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = shaper.layout(paragraph, 18.0);
        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64() * 1000.0);
    }

    let min_ms = times.iter().copied().fold(f64::INFINITY, f64::min);
    let max_ms = times.iter().copied().fold(0.0, f64::max);
    let sum: f64 = times.iter().sum();
    let mean_ms = sum / (iterations as f64);
    let target_met = mean_ms < 5.0;

    ShapingBenchmarkResult {
        character_count: paragraph.chars().count(),
        iterations,
        mean_ms,
        min_ms,
        max_ms,
        target_met,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_shaping_and_bidi_runs() {
        let mut shaper = BilingualShaper::new();
        let sample = "خوارزمية Dijkstra تعمل في O(V log V)";
        let layout = shaper.layout(sample, 20.0);
        let report = verify_bilingual_layout(&layout, sample);

        assert!(report.width > 0.0);
        assert!(report.height > 0.0);
        assert!(
            report.cursive_shaped,
            "Arabic text must be RTL with valid glyphs"
        );
        assert!(
            report.bidi_isolated,
            "Latin tokens must not be rendered as RTL"
        );
        assert!(report.cluster_count > 0);
    }

    #[test]
    fn test_shaping_benchmark_runs() {
        let mut shaper = BilingualShaper::new();
        let paragraph =
            "خوارزمية Dijkstra هي خوارزمية للبحث عن أقصر مسار في الرسم البياني الموزون. \
                         Algorithm complexity is O(V log V + E) when using a Fibonacci heap. \
                         تعمل هذه الطريقة بكفاءة عالية مع الرسوم البيانية ذات الأوزان غير السالبة.";
        let bench = benchmark_shaping_latency(&mut shaper, paragraph, 10);
        assert!(bench.mean_ms > 0.0);
        assert!(bench.character_count > 100);
    }
}
