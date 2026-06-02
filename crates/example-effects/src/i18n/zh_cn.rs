//! Chinese (zh_CN) translations for VideoFX Example ExTrKey variants.

use std::ffi::CStr;
use crate::i18n::ExTrKey;

pub fn translate_cstr(key: ExTrKey) -> &'static CStr {
    match key {
        // Effect display names
        ExTrKey::ParamColorAdjustmentName => c"VideoFX 示例 - 颜色调整",
        ExTrKey::ParamSolidBlendName => c"VideoFX 示例 - 纯色混合",

        // SolidColorBlend param labels
        ExTrKey::ParamColor => c"颜色",
        ExTrKey::ParamColorDesc => c"纯色和混合量 (RGBA)。",
        ExTrKey::ParamColorRed => c"颜色 - 红",
        ExTrKey::ParamColorRedDesc => c"纯色的红色分量。",
        ExTrKey::ParamColorGreen => c"颜色 - 绿",
        ExTrKey::ParamColorGreenDesc => c"纯色的绿色分量。",
        ExTrKey::ParamColorBlue => c"颜色 - 蓝",
        ExTrKey::ParamColorBlueDesc => c"纯色的蓝色分量。",
        ExTrKey::ParamBlendAmount => c"混合量",
        ExTrKey::ParamBlendAmountDesc => c"Alpha 通道混合。0% = 原始图像，100% = 纯色。",
        ExTrKey::ParamBlendAttenuation => c"混合衰减",
        ExTrKey::ParamBlendAttenuationDesc => c"衰减 Alpha 通道上的混合效果。100% = 无混合（Alpha 直接通过），0% = 完全混合。",
        ExTrKey::ParamBlendMode => c"混合模式",
        ExTrKey::ParamBlendModeDesc => c"纯色与图像的混合方式。",

        // SolidColorBlend menu item labels
        ExTrKey::MenuNormal => c"正常",
        ExTrKey::MenuMultiply => c"正片叠底",
        ExTrKey::MenuScreen => c"滤色",
        ExTrKey::MenuOverlay => c"叠加",
        ExTrKey::MenuNormalDesc => c"图像与纯色之间的线性插值。",
        ExTrKey::MenuMultiplyDesc => c"将图像乘以纯色。",
        ExTrKey::MenuScreenDesc => c"用纯色对图像进行滤色（反向乘法）。",
        ExTrKey::MenuOverlayDesc => c"基于图像亮度结合正片叠底和滤色。",

        // ColorAdjustment param labels
        ExTrKey::ParamBrightness => c"亮度",
        ExTrKey::ParamBrightnessDesc => c"整体亮度倍增器。",
        ExTrKey::ParamInvertColors => c"反转颜色",
        ExTrKey::ParamInvertColorsDesc => c"反转图像中的所有颜色。",
        ExTrKey::ParamTint => c"色调",
        ExTrKey::ParamTintDesc => c"各通道色调倍增器 (RGB)。",
        ExTrKey::ParamTintRed => c"色调 - 红",
        ExTrKey::ParamTintRedDesc => c"红色通道色调倍增器。",
        ExTrKey::ParamTintGreen => c"色调 - 绿",
        ExTrKey::ParamTintGreenDesc => c"绿色通道色调倍增器。",
        ExTrKey::ParamTintBlue => c"色调 - 蓝",
        ExTrKey::ParamTintBlueDesc => c"蓝色通道色调倍增器。",
        ExTrKey::ParamAdvanced => c"高级",
        ExTrKey::ParamAdvancedDesc => c"其他高级设置。",
        ExTrKey::ParamContrast => c"对比度",
        ExTrKey::ParamContrastDesc => c"对比度调整。",
        ExTrKey::ParamSaturation => c"饱和度",
        ExTrKey::ParamSaturationDesc => c"颜色饱和度调整。",
        ExTrKey::ParamColorPreset => c"颜色预设",
        ExTrKey::ParamColorPresetDesc => c"选择颜色预设。",
        ExTrKey::MenuNone => c"无",
        ExTrKey::MenuNoneDesc => c"无颜色预设。",
        ExTrKey::MenuWarm => c"暖色",
        ExTrKey::MenuWarmDesc => c"暖色调。",
        ExTrKey::MenuCool => c"冷色",
        ExTrKey::MenuCoolDesc => c"冷色调。",
        ExTrKey::MenuSepia => c"怀旧",
        ExTrKey::MenuSepiaDesc => c"怀旧色调。",
    }
}
