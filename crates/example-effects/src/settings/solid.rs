use video_fx_macros::FullSettings;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{ExTrKey, MenuItem, SettingDescriptor, SettingKind, Settings, SettingsEnum};

// ---------------------------------------------------------------------------
// Blend mode enum
// ---------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum BlendMode {
    Normal = 0,
    Multiply,
    Screen,
    Overlay,
}
impl SettingsEnum for BlendMode {}

// ---------------------------------------------------------------------------
// Settings struct
// ---------------------------------------------------------------------------

/// Settings for the solid-color-blend effect.
///
/// The solid color is stored as RGBA where:
/// - `color_r`, `color_g`, `color_b` are the solid color channels (0–1)
/// - `color_a` doubles as the blend amount (0 = full original, 1 = full solid color)
/// - `blend_attenuation` attenuates the blend effect on the alpha channel (1.0 = no blend, 0.0 = full blend)
#[derive(FullSettings, Clone, Debug, PartialEq)]
pub struct SolidColorBlend {
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
    pub blend_attenuation: f32,
    pub blend_mode: BlendMode,
}

impl Default for SolidColorBlend {
    fn default() -> Self {
        Self {
            color_r: 1.0,
            color_g: 1.0,
            color_b: 1.0,
            color_a: 1.0,
            blend_attenuation: 1.0,
            blend_mode: BlendMode::Normal,
        }
    }
}

// ---------------------------------------------------------------------------
// Setting IDs
// ---------------------------------------------------------------------------

#[rustfmt::skip]
pub mod setting_id {
    use crate::{setting_id, settings::SettingID};
    use super::SolidColorBlendFullSettings;
    type SID = SettingID<SolidColorBlendFullSettings>;

    pub const COLOR:        SID = setting_id!("color", color_r);
    pub const COLOR_R:      SID = setting_id!("color_r", color_r);
    pub const COLOR_G:      SID = setting_id!("color_g", color_g);
    pub const COLOR_B:      SID = setting_id!("color_b", color_b);
    pub const COLOR_A:      SID = setting_id!("color_a", color_a);
    pub const BLEND_ATTENUATION: SID = setting_id!("blend_attenuation", blend_attenuation);
    pub const BLEND_MODE:   SID = setting_id!("blend_mode", blend_mode);
}

// ---------------------------------------------------------------------------
// Settings trait impl
// ---------------------------------------------------------------------------

impl Settings for SolidColorBlendFullSettings {
    type Key = ExTrKey;

    fn setting_descriptors() -> Box<[SettingDescriptor<Self>]> {
        vec![
            SettingDescriptor {
                label_key: ExTrKey::ParamColor,
                description_key: Some(ExTrKey::ParamColorDesc),
                kind: SettingKind::ColorRGBA {
                    r_id: setting_id::COLOR_R,
                    g_id: setting_id::COLOR_G,
                    b_id: setting_id::COLOR_B,
                    a_id: setting_id::COLOR_A,
                },
                id: setting_id::COLOR,
            },
            SettingDescriptor {
                label_key: ExTrKey::ParamBlendAttenuation,
                description_key: Some(ExTrKey::ParamBlendAttenuationDesc),
                kind: SettingKind::Percentage { logarithmic: false },
                id: setting_id::BLEND_ATTENUATION,
            },
            SettingDescriptor {
                label_key: ExTrKey::ParamBlendMode,
                description_key: Some(ExTrKey::ParamBlendModeDesc),
                kind: SettingKind::Enumeration {
                    options: vec![
                        MenuItem {
                            label_key: ExTrKey::MenuNormal,
                            description_key: Some(ExTrKey::MenuNormalDesc),
                            index: BlendMode::Normal as u32,
                        },
                        MenuItem {
                            label_key: ExTrKey::MenuMultiply,
                            description_key: Some(ExTrKey::MenuMultiplyDesc),
                            index: BlendMode::Multiply as u32,
                        },
                        MenuItem {
                            label_key: ExTrKey::MenuScreen,
                            description_key: Some(ExTrKey::MenuScreenDesc),
                            index: BlendMode::Screen as u32,
                        },
                        MenuItem {
                            label_key: ExTrKey::MenuOverlay,
                            description_key: Some(ExTrKey::MenuOverlayDesc),
                            index: BlendMode::Overlay as u32,
                        },
                    ],
                },
                id: setting_id::BLEND_MODE,
            },
        ]
        .into_boxed_slice()
    }

    fn legacy_value() -> Self {
        Default::default()
    }
}
