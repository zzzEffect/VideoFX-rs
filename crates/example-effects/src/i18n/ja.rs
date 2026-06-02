//! Japanese (ja) translations for VideoFX Example ExTrKey variants.

use std::ffi::CStr;
use crate::i18n::ExTrKey;

pub fn translate_cstr(key: ExTrKey) -> &'static CStr {
    match key {
        // Effect display names
        ExTrKey::ParamColorAdjustmentName => c"VideoFX Example 色調整",
        ExTrKey::ParamSolidBlendName => c"VideoFX Example ソリッドブレンド",

        // SolidColorBlend param labels
        ExTrKey::ParamColor => c"色",
        ExTrKey::ParamColorDesc => c"単色とブレンド量 (RGBA)。",
        ExTrKey::ParamColorRed => c"色 - 赤",
        ExTrKey::ParamColorRedDesc => c"単色の赤成分。",
        ExTrKey::ParamColorGreen => c"色 - 緑",
        ExTrKey::ParamColorGreenDesc => c"単色の緑成分。",
        ExTrKey::ParamColorBlue => c"色 - 青",
        ExTrKey::ParamColorBlueDesc => c"単色の青成分。",
        ExTrKey::ParamBlendAmount => c"ブレンド量",
        ExTrKey::ParamBlendAmountDesc => c"アルファチャンネルブレンド。0% = 元画像、100% = 単色。",
        ExTrKey::ParamBlendAttenuation => c"ブレンド減衰",
        ExTrKey::ParamBlendAttenuationDesc => c"アルファチャンネルのブレンド効果を減衰します。100% = ブレンドなし（アルファはそのまま）、0% = 完全ブレンド。",
        ExTrKey::ParamBlendMode => c"ブレンドモード",
        ExTrKey::ParamBlendModeDesc => c"単色と画像のブレンド方法。",

        // SolidColorBlend menu item labels
        ExTrKey::MenuNormal => c"通常",
        ExTrKey::MenuMultiply => c"乗算",
        ExTrKey::MenuScreen => c"スクリーン",
        ExTrKey::MenuOverlay => c"オーバーレイ",
        ExTrKey::MenuNormalDesc => c"画像と単色の線形補間。",
        ExTrKey::MenuMultiplyDesc => c"画像に単色を乗算します。",
        ExTrKey::MenuScreenDesc => c"単色でスクリーン効果（乗算の逆）。",
        ExTrKey::MenuOverlayDesc => c"画像の明るさに基づいて乗算とスクリーンを組み合わせます。",

        // ColorAdjustment param labels
        ExTrKey::ParamBrightness => c"明るさ",
        ExTrKey::ParamBrightnessDesc => c"全体の明るさの乗数。",
        ExTrKey::ParamInvertColors => c"色反転",
        ExTrKey::ParamInvertColorsDesc => c"画像のすべての色を反転します。",
        ExTrKey::ParamTint => c"色合い",
        ExTrKey::ParamTintDesc => c"各チャンネルの色合い乗数 (RGB)。",
        ExTrKey::ParamTintRed => c"色合い - 赤",
        ExTrKey::ParamTintRedDesc => c"赤チャンネルの色合い乗数。",
        ExTrKey::ParamTintGreen => c"色合い - 緑",
        ExTrKey::ParamTintGreenDesc => c"緑チャンネルの色合い乗数。",
        ExTrKey::ParamTintBlue => c"色合い - 青",
        ExTrKey::ParamTintBlueDesc => c"青チャンネルの色合い乗数。",
        ExTrKey::ParamAdvanced => c"詳細設定",
        ExTrKey::ParamAdvancedDesc => c"追加の詳細設定。",
        ExTrKey::ParamContrast => c"コントラスト",
        ExTrKey::ParamContrastDesc => c"コントラスト調整。",
        ExTrKey::ParamSaturation => c"彩度",
        ExTrKey::ParamSaturationDesc => c"色の彩度調整。",
        ExTrKey::ParamColorPreset => c"カラープリセット",
        ExTrKey::ParamColorPresetDesc => c"カラープリセットを選択します。",
        ExTrKey::MenuNone => c"なし",
        ExTrKey::MenuNoneDesc => c"カラープリセットなし。",
        ExTrKey::MenuWarm => c"暖色",
        ExTrKey::MenuWarmDesc => c"暖かい色調。",
        ExTrKey::MenuCool => c"寒色",
        ExTrKey::MenuCoolDesc => c"冷たい色調。",
        ExTrKey::MenuSepia => c"セピア",
        ExTrKey::MenuSepiaDesc => c"セピア色調。",
    }
}
