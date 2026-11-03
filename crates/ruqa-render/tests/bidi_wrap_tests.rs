use ruqa_render::text_layout::{
    verify_bilingual_layout, wrap_ltr_isolate, BilingualShaper, LRI, PDI,
};

#[test]
fn test_bidi_line_wrapping_preserves_cursive_connections() {
    let mut shaper = BilingualShaper::new();
    let text = "خوارزمية Dijkstra هي خوارزمية للبحث عن أقصر مسار في الرسم البياني الموزون";

    let unwrapped = shaper.layout(text, 18.0);
    assert_eq!(unwrapped.lines().count(), 1);

    let wrapped = shaper.layout_with_max_width(text, 18.0, Some(220.0));
    assert!(
        wrapped.lines().count() >= 2,
        "Should wrap into multiple lines"
    );

    let report = verify_bilingual_layout(&wrapped, text);
    assert!(
        report.cursive_shaped,
        "Arabic words must remain cursive shaped when wrapped"
    );
    assert!(
        report.bidi_isolated,
        "Latin words must remain isolated from Arabic RTL"
    );
}

#[test]
fn test_bidi_line_wrapping_keeps_ltr_isolates_intact() {
    let mut shaper = BilingualShaper::new();
    let token = wrap_ltr_isolate("O(V log V)");
    let sentence = format!("تعقيد الخوارزمية الزمني هو {token} في الحالة الأسوأ");

    let wrapped = shaper.layout_with_max_width(&sentence, 20.0, Some(200.0));
    assert!(wrapped.lines().count() >= 2);

    // Verify isolate characters were preserved in input
    assert!(sentence.contains(LRI));
    assert!(sentence.contains(PDI));

    let report = verify_bilingual_layout(&wrapped, &sentence);
    assert!(report.cursive_shaped);
    assert!(report.cluster_count > 0);
}

#[test]
fn test_bidi_multiline_monotonic_baselines() {
    let mut shaper = BilingualShaper::new();
    let paragraph =
        "هذا نص تجريبي باللغة العربية مع كلمات English ورموز رياضية 12345 للتأكد من تتابع السطور";
    let wrapped = shaper.layout_with_max_width(paragraph, 16.0, Some(150.0));

    let mut previous_baseline = -1.0;
    let mut line_count = 0;

    for line in wrapped.lines() {
        line_count += 1;
        for item in line.items() {
            if let parley::layout::PositionedLayoutItem::GlyphRun(run) = item {
                let baseline = run.baseline();
                assert!(
                    baseline >= previous_baseline,
                    "Line baselines must monotonically increase down the block axis"
                );
                previous_baseline = baseline;
            }
        }
    }

    assert!(line_count >= 3, "Expected at least 3 wrapped lines");
    assert!(wrapped.height() > 40.0);
}
