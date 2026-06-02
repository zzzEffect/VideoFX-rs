use example_effects::ColorAdjustment;

#[test]
fn default_parameters_are_identity() {
    let effect = ColorAdjustment::default();

    let width = 4;
    let height = 2;
    let len = width * height * 4;

    let src: Vec<u8> = (0..len).map(|i| i as u8).collect();
    let mut dst = vec![0u8; len];

    effect.apply_effect(&src, &mut dst, width, height);

    assert_eq!(src, dst, "default parameters should produce identity (no-op)");
}

#[test]
fn effect_actually_transforms_pixels() {
    let mut effect = ColorAdjustment::default();
    effect.brightness = 0.5;
    effect.invert_colors = true;
    effect.color_preset = example_effects::settings::color_adjustment::ColorPreset::Sepia;

    let width = 2;
    let height = 2;
    let len = width * height * 4;

    let src: Vec<u8> = (0..len).map(|i| (i * 17) as u8).collect();
    let mut dst = vec![0u8; len];

    effect.apply_effect(&src, &mut dst, width, height);

    // With non-identity parameters, the output should differ from the input
    assert_ne!(src, dst, "non-default parameters should modify pixels");
}

#[test]
fn different_dimensions() {
    let effect = ColorAdjustment::default();

    for (w, h) in [(1, 1), (16, 9), (64, 64), (1920, 1080)] {
        let len = w * h * 4;
        let src: Vec<u8> = (0..len).map(|i| (i % 256) as u8).collect();
        let mut dst = vec![0u8; len];
        effect.apply_effect(&src, &mut dst, w, h);
        assert_eq!(src, dst, "default identity failed for {w}x{h}");
    }
}
