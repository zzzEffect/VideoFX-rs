use std::ffi::OsStr;

#[rustfmt::skip]
fn main() {
    let is_ae = std::env::var_os("CARGO_CFG_WINDOWS").is_some()
        || std::env::var_os("CARGO_CFG_TARGET_OS").as_deref() == Some(OsStr::new("macos"));
    if !is_ae {
        return;
    }

    const PF_PLUG_IN_VERSION: u16 = 13;
    const PF_PLUG_IN_SUBVERS: u16 = 28;
    const EFFECT_VERSION_MAJOR: u32 = 0;
    const EFFECT_VERSION_MINOR: u32 = 1;
    const EFFECT_VERSION_PATCH: u32 = 0;

    use pipl::*;

    fn to_seq(bytes: &[u8]) -> String {
        bytes.iter().fold(String::new(), |mut s, b| {
            s.push_str(&format!("\\x{b:02x}"));
            s
        })
    }

    // ── Primary effect via pipl::plugin_build — sets ALL correct env vars ─

    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("VideoFX Example Color Adjustment"),
        Property::Category("VideoFX Example"),

        #[cfg(target_os = "windows")]
        Property::CodeWin64X86("EffectMainColorAdjustment"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EffectMainColorAdjustment"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EffectMainColorAdjustment"),

        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version { major: PF_PLUG_IN_VERSION, minor: PF_PLUG_IN_SUBVERS },
        Property::AE_Effect_Version {
            version: EFFECT_VERSION_MAJOR,
            subversion: EFFECT_VERSION_MINOR,
            bugversion: EFFECT_VERSION_PATCH,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
            OutFlags::NonParamVary |
            OutFlags::DeepColorAware |
            OutFlags::SendUpdateParamsUI |
            OutFlags::PiplOverridesOutdataOutflags
        ),
        Property::AE_Effect_Global_OutFlags_2(
            OutFlags2::ParamGroupStartCollapsedFlag |
            OutFlags2::SupportsSmartRender |
            OutFlags2::FloatColorAware |
            OutFlags2::RevealsZeroAlpha |
            OutFlags2::SupportsThreadedRendering |
            OutFlags2::SupportsGetFlattenedSequenceData
        ),
        Property::AE_Effect_Match_Name("video-fx-example-color-adjustment"),
        Property::AE_Reserved_Info(8),
        Property::AE_Effect_Support_URL("https://example.com/plugin"),
    ]);

    // ── Rebuild both PiPLs into a single winres compile ─────────────────
    // (pipl::plugin_build already compiled resource 16000; we overwrite with
    // a single compile that contains both 16000 + 16001 in one .res file.)

    #[cfg(target_os = "windows")]
    {
        #[rustfmt::skip]
        fn build_effect_pipl(
            name: &'static str,
            match_name: &'static str,
            entry_name: &'static str,
        ) -> Vec<Property> {
            vec![
                Property::Kind(PIPLType::AEEffect),
                Property::Name(name),
                Property::Category("VideoFX Example"),
                Property::CodeWin64X86(entry_name),
                Property::AE_PiPL_Version { major: 2, minor: 0 },
                Property::AE_Effect_Spec_Version { major: PF_PLUG_IN_VERSION, minor: PF_PLUG_IN_SUBVERS },
                Property::AE_Effect_Version {
                    version: EFFECT_VERSION_MAJOR,
                    subversion: EFFECT_VERSION_MINOR,
                    bugversion: EFFECT_VERSION_PATCH,
                    stage: Stage::Develop, build: 1,
                },
                Property::AE_Effect_Info_Flags(0),
                Property::AE_Effect_Global_OutFlags(
                    OutFlags::NonParamVary | OutFlags::DeepColorAware |
                    OutFlags::SendUpdateParamsUI | OutFlags::PiplOverridesOutdataOutflags
                ),
                Property::AE_Effect_Global_OutFlags_2(
                    OutFlags2::ParamGroupStartCollapsedFlag | OutFlags2::SupportsSmartRender |
                    OutFlags2::FloatColorAware | OutFlags2::RevealsZeroAlpha |
                    OutFlags2::SupportsThreadedRendering | OutFlags2::SupportsGetFlattenedSequenceData
                ),
                Property::AE_Effect_Match_Name(match_name),
                Property::AE_Reserved_Info(8),
                Property::AE_Effect_Support_URL("https://example.com/plugin"),
            ]
        }

        let effects: &[(&str, &str, &str)] = &[
            ("VideoFX Example Color Adjustment", "video-fx-example-color-adjustment", "EffectMainColorAdjustment"),
            ("VideoFX Example Solid Blend", "video-fx-example-solid-blend", "EffectMainSolidBlend"),
        ];

        let mut res = winres::WindowsResource::new();
        for (idx, (name, match_name, entry_name)) in effects.iter().enumerate() {
            let props = build_effect_pipl(name, match_name, entry_name);
            let pipl = build_pipl(props).unwrap();
            let id = 16000 + idx as i16;
            res.append_rc_content(&format!(
                "{id} PiPL DISCARDABLE BEGIN \"{}\" END\n",
                to_seq(&pipl)
            ));
        }
        res.compile().unwrap();
    }

    // pipl::plugin_build hardcodes PIPL_ENTRYPOINT=EffectMain as a fallback;
    // override it to match our wrapper.
    println!("cargo:rustc-env=PIPL_ENTRYPOINT=EffectMainColorAdjustment");

    println!("cargo:rustc-env=EFFECT_VERSION_MAJOR={EFFECT_VERSION_MAJOR}");
    println!("cargo:rustc-env=EFFECT_VERSION_MINOR={EFFECT_VERSION_MINOR}");
    println!("cargo:rustc-env=EFFECT_VERSION_PATCH={EFFECT_VERSION_PATCH}");
    println!("cargo:rustc-cfg=with_premiere");
    println!("cargo:rustc-cfg=catch_panics");
}
