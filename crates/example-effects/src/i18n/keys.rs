use std::ffi::CStr;

use crate::i18n_keys;

i18n_keys! {
    pub ExTrKey {
        // Effect display names
        ParamColorAdjustmentName = "VideoFX Example Color Adjustment";
        ParamSolidBlendName = "VideoFX Example Solid Blend";

        // SolidColorBlend param labels
        ParamColor = "Color";
        ParamColorDesc = "Solid color and blend amount (RGBA).";
        ParamColorRed = "Color Red";
        ParamColorRedDesc = "Red component of the solid color.";
        ParamColorGreen = "Color Green";
        ParamColorGreenDesc = "Green component of the solid color.";
        ParamColorBlue = "Color Blue";
        ParamColorBlueDesc = "Blue component of the solid color.";
        ParamBlendAmount = "Blend Amount";
        ParamBlendAmountDesc = "Alpha channel blending. 0% = original image, 100% = solid color.";
        ParamBlendAttenuation = "Blend Attenuation";
        ParamBlendAttenuationDesc = "Attenuates the blend effect on the alpha channel. 100% = no blend (alpha passes through), 0% = full blend.";
        ParamBlendMode = "Blend Mode";
        ParamBlendModeDesc = "How the solid color is blended with the image.";

        // SolidColorBlend menu item labels
        MenuNormal = "Normal";
        MenuMultiply = "Multiply";
        MenuScreen = "Screen";
        MenuOverlay = "Overlay";
        MenuNormalDesc = "Linear interpolation between image and solid color.";
        MenuMultiplyDesc = "Multiplies the image by the solid color.";
        MenuScreenDesc = "Screens the image with the solid color (inverse multiply).";
        MenuOverlayDesc = "Combines Multiply and Screen based on image brightness.";

        // ColorAdjustment param labels
        ParamBrightness = "Brightness";
        ParamBrightnessDesc = "Overall brightness multiplier.";
        ParamInvertColors = "Invert Colors";
        ParamInvertColorsDesc = "Invert all colors in the image.";
        ParamTint = "Tint";
        ParamTintDesc = "Per-channel tint multipliers (RGB).";
        ParamTintRed = "Tint Red";
        ParamTintRedDesc = "Red channel tint multiplier.";
        ParamTintGreen = "Tint Green";
        ParamTintGreenDesc = "Green channel tint multiplier.";
        ParamTintBlue = "Tint Blue";
        ParamTintBlueDesc = "Blue channel tint multiplier.";
        ParamAdvanced = "Advanced";
        ParamAdvancedDesc = "Additional advanced settings.";
        ParamContrast = "Contrast";
        ParamContrastDesc = "Contrast adjustment.";
        ParamSaturation = "Saturation";
        ParamSaturationDesc = "Color saturation adjustment.";
        ParamColorPreset = "Color Preset";
        ParamColorPresetDesc = "Choose a color preset.";
        MenuNone = "None";
        MenuNoneDesc = "No color preset.";
        MenuWarm = "Warm";
        MenuWarmDesc = "Warm color tone.";
        MenuCool = "Cool";
        MenuCoolDesc = "Cool color tone.";
        MenuSepia = "Sepia";
        MenuSepiaDesc = "Sepia color tone.";
    }
}
