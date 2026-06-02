use example_effects::{
    SolidColorBlend, SolidColorBlendFullSettings,
    settings::Settings,
    settings::solid::BlendMode,
};

// ---------------------------------------------------------------------------
// Normal blend
// ---------------------------------------------------------------------------

#[test]
fn zero_alpha_is_passthrough() {
    let effect = SolidColorBlend {
        color_a: 0.0,
        color_r: 0.8,
        color_g: 0.2,
        color_b: 0.5,
        ..Default::default()
    };

    let src: Vec<u8> = (0..16).map(|i| i as u8).collect();
    let mut dst = vec![0u8; src.len()];
    effect.apply_effect(&src, &mut dst, 2, 2);
    assert_eq!(src, dst, "zero alpha should be identity");
}

#[test]
fn full_alpha_is_solid_color_normal() {
    let effect = SolidColorBlend {
        blend_mode: BlendMode::Normal,
        color_a: 1.0,
        color_r: 1.0,
        color_g: 0.0,
        color_b: 0.5,
        blend_attenuation: 1.0,
    };

    let width = 2;
    let height = 2;
    let len = width * height * 4;
    let src = vec![50u8; len];
    let mut dst = vec![0u8; len];
    effect.apply_effect(&src, &mut dst, width, height);

    for i in (0..len).step_by(4) {
        assert_eq!(dst[i],     255, "red at pixel {}", i / 4);
        assert_eq!(dst[i + 1], 0,   "green at pixel {}", i / 4);
        assert_eq!(dst[i + 2], 128, "blue at pixel {}", i / 4);
        assert_eq!(dst[i + 3], src[i + 3], "alpha preserved");
    }
}

#[test]
fn alpha_blended_when_blend_attenuation_is_0() {
    // With blend_attenuation=0, alpha fully participates in the blend
    let effect = SolidColorBlend {
        blend_mode: BlendMode::Normal,
        color_a: 1.0,
        color_r: 0.5,
        color_g: 0.5,
        color_b: 0.5,
        blend_attenuation: 0.0,
    };
    let src = vec![0u8, 0, 0, 128];
    let mut dst = vec![0u8; 4];
    effect.apply_effect(&src, &mut dst, 1, 1);
    // Alpha: src_a=0.5, blended=1.0, effective=0.5+(1.0-0.5)*(1-0)=1.0 → 255
    assert_eq!(dst[3], 255);
}

#[test]
fn alpha_preserved_when_blend_attenuation_is_1() {
    let effect = SolidColorBlend {
        color_a: 0.3,
        color_r: 0.5,
        color_g: 0.5,
        color_b: 0.5,
        blend_attenuation: 1.0,
        ..Default::default()
    };
    let src: Vec<u8> = vec![0, 0, 0, 77];
    let mut dst = vec![0u8; 4];
    effect.apply_effect(&src, &mut dst, 1, 1);
    assert_eq!(dst[3], 77, "alpha must be preserved when blend_attenuation=1");
}

// ---------------------------------------------------------------------------
// Blend mode tests
// ---------------------------------------------------------------------------

#[test]
fn multiply_darkens() {
    let effect = SolidColorBlend {
        blend_mode: BlendMode::Multiply,
        color_a: 1.0,
        color_r: 0.5,
        color_g: 0.5,
        color_b: 0.5,
        blend_attenuation: 1.0,
    };
    let src = vec![255u8, 255, 255, 255];
    let mut dst = vec![0u8; 4];
    effect.apply_effect(&src, &mut dst, 1, 1);

    assert_eq!(dst[0], 128);
    assert_eq!(dst[1], 128);
    assert_eq!(dst[2], 128);
    assert_eq!(dst[3], 255);
}

#[test]
fn screen_lightens() {
    let effect = SolidColorBlend {
        blend_mode: BlendMode::Screen,
        color_a: 1.0,
        color_r: 1.0,
        color_g: 1.0,
        color_b: 1.0,
        blend_attenuation: 1.0,
    };
    let src = vec![0u8, 0, 0, 255];
    let mut dst = vec![0u8; 4];
    effect.apply_effect(&src, &mut dst, 1, 1);
    assert_eq!(dst[0], 255);
    assert_eq!(dst[1], 255);
    assert_eq!(dst[2], 255);
}

#[test]
fn overlay_on_gray_is_unchanged_color() {
    let effect = SolidColorBlend {
        blend_mode: BlendMode::Overlay,
        color_a: 1.0,
        color_r: 0.5,
        color_g: 0.5,
        color_b: 0.5,
        blend_attenuation: 1.0,
    };
    let src = vec![128u8, 128, 128, 255];
    let mut dst = vec![0u8; 4];
    effect.apply_effect(&src, &mut dst, 1, 1);

    assert!((dst[0] as i32 - 128).abs() <= 1, "should be ~128, got {}", dst[0]);
    assert!((dst[1] as i32 - 128).abs() <= 1, "should be ~128, got {}", dst[1]);
    assert!((dst[2] as i32 - 128).abs() <= 1, "should be ~128, got {}", dst[2]);
}

#[test]
fn different_dimensions_work() {
    for &mode in &[BlendMode::Normal, BlendMode::Multiply, BlendMode::Screen, BlendMode::Overlay] {
        let effect = SolidColorBlend {
            blend_mode: mode,
            color_a: 0.25,
            color_r: 1.0,
            color_g: 1.0,
            color_b: 1.0,
            blend_attenuation: 1.0,
        };
        for (w, h) in [(1, 1), (3, 2)] {
            let len = w * h * 4;
            let src: Vec<u8> = (0..len).map(|i| (i % 256) as u8).collect();
            let mut dst = vec![0u8; len];
            effect.apply_effect(&src, &mut dst, w, h);
            for p in 0..len / 4 {
                assert_eq!(dst[p * 4 + 3], src[p * 4 + 3], "alpha for mode {mode:?} at pixel {p}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Settings tests
// ---------------------------------------------------------------------------

#[test]
fn solid_blend_descriptors_count() {
    let list = example_effects::settings::SettingsList::<SolidColorBlendFullSettings>::new();
    let all: Vec<_> = list.all_descriptors().collect();
    // 3 descriptors: ColorRGBA, blend_attenuation, blend_mode
    assert_eq!(all.len(), 3, "should have 3 descriptors");
}

#[test]
fn solid_blend_default_is_white_full_blend() {
    let default = SolidColorBlend::default();
    assert_eq!(default.color_r, 1.0);
    assert_eq!(default.color_g, 1.0);
    assert_eq!(default.color_b, 1.0);
    assert_eq!(default.color_a, 1.0);
    assert_eq!(default.blend_attenuation, 1.0);
    assert_eq!(default.blend_mode, BlendMode::Normal);
}

#[test]
fn solid_blend_json_round_trip() {
    use example_effects::settings::SettingsList;
    use example_effects::settings::solid::setting_id;

    let list = SettingsList::<SolidColorBlendFullSettings>::new();

    let mut settings = SolidColorBlendFullSettings::default();
    settings.set_field::<f32>(&setting_id::COLOR_R, 0.5).unwrap();
    settings.set_field::<f32>(&setting_id::COLOR_G, 0.2).unwrap();
    settings.set_field::<f32>(&setting_id::COLOR_B, 0.9).unwrap();
    settings.set_field::<f32>(&setting_id::COLOR_A, 0.75).unwrap();
    settings.set_field::<f32>(&setting_id::BLEND_ATTENUATION, 0.3).unwrap();
    settings.set_field::<example_effects::settings::EnumValue>(
        &setting_id::BLEND_MODE,
        example_effects::settings::EnumValue(BlendMode::Screen as u32),
    ).unwrap();

    let json = list.to_json_string(&settings).unwrap();
    let parsed = list.from_json_generic(&json).unwrap();

    assert_eq!(parsed.get_field::<f32>(&setting_id::COLOR_R).unwrap(), 0.5);
    assert_eq!(parsed.get_field::<f32>(&setting_id::COLOR_G).unwrap(), 0.2);
    assert_eq!(parsed.get_field::<f32>(&setting_id::COLOR_B).unwrap(), 0.9);
    assert_eq!(parsed.get_field::<f32>(&setting_id::COLOR_A).unwrap(), 0.75);
    assert_eq!(parsed.get_field::<f32>(&setting_id::BLEND_ATTENUATION).unwrap(), 0.3);
    assert_eq!(
        parsed.get_field::<example_effects::settings::EnumValue>(&setting_id::BLEND_MODE).unwrap().0,
        BlendMode::Screen as u32
    );
}
