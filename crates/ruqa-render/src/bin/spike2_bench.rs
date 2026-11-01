//! Standalone runner for Spike 2: Arabic and bilingual text shaping with Parley.

use ruqa_render::text_layout::{
    benchmark_shaping_latency, verify_bilingual_layout, BilingualShaper,
};

fn main() {
    println!("=== Ruqa Spike 2: Arabic & Bilingual Text Shaping with Parley ===");
    println!(
        "Testing fonts: Aref Ruqaa Regular (Arabic OFL) & Roboto Regular (Latin Apache-2.0)\n"
    );

    let mut shaper = BilingualShaper::new();

    // 1. Mandatory mixed test string from Issue #4
    let test_string = "خوارزمية Dijkstra تعمل في O(V log V)";
    println!("Target Test String: \"{}\"", test_string);

    let layout = shaper.layout(test_string, 24.0);
    let report = verify_bilingual_layout(&layout, test_string);

    println!("--------------------------------------------------");
    println!(
        "Layout Dimensions: {:.2} x {:.2} px",
        report.width, report.height
    );
    println!("Total Clusters   : {}", report.cluster_count);
    println!("Runs Breakdown   :");
    for (i, run) in report.runs.iter().enumerate() {
        println!(
            "  [{}] slice: \"{}\" | dir: {} | glyphs: {} | advance: {:.2}px",
            i,
            run.slice,
            if run.is_rtl { "RTL" } else { "LTR" },
            run.glyph_count,
            run.advance
        );
    }
    println!("Arabic Cursive Connected: {}", report.cursive_shaped);
    println!("Latin Tokens Isolated   : {}", report.bidi_isolated);
    println!("--------------------------------------------------\n");

    // 2. 500-character bilingual paragraph benchmark
    let paragraph = "خوارزمية ديجكسترا (Dijkstra's algorithm) هي خوارزمية حاسوبية كلاسيكية طوّرها \
                     عالم الحاسوب إدسخر ديجكسترا في عام 1956. تستخدم لإيجاد أقصر مسار بين عقدتين \
                     في رسم بياني متصل بوزن غير سالب (non-negative edge weights). \
                     التعقيد الزمني القياسي للخوارزمية هو O(V^2) عند استخدام مصفوفة المجاورة، \
                     ويمكن تحسينه إلى O(E + V log V) عند استخدام كومة فيبوناتشي (Fibonacci heap). \
                     تعتبر هذه الخوارزمية حجر الأساس في بروتوكولات توجيه الشبكات مثل OSPF وIS-IS.";

    println!("Running Paragraph Shaping Latency Benchmark (500 characters, 100 iterations)...");
    let bench = benchmark_shaping_latency(&mut shaper, paragraph, 100);

    println!("--------------------------------------------------");
    println!("Character Count : {}", bench.character_count);
    println!("Iterations      : {}", bench.iterations);
    println!("Min Latency     : {:.3} ms", bench.min_ms);
    println!("Mean Latency    : {:.3} ms", bench.mean_ms);
    println!("Max Latency     : {:.3} ms", bench.max_ms);
    println!(
        "Target Met (<5ms): {} ({:.3} ms)",
        bench.target_met, bench.mean_ms
    );
    println!("--------------------------------------------------");
}
