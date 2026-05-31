use std::ffi::CStr;

use crate::i18n_keys;

i18n_keys! {
    pub ExTrKey {
        // SolidColorBlend param labels
        ParamColorRed = "Color Red";
        ParamColorRedDesc = "Red component of the solid color.";
        ParamColorGreen = "Color Green";
        ParamColorGreenDesc = "Green component of the solid color.";
        ParamColorBlue = "Color Blue";
        ParamColorBlueDesc = "Blue component of the solid color.";
        ParamBlendAmount = "Blend Amount";
        ParamBlendAmountDesc = "Alpha channel blending. 0% = original image, 100% = solid color.";
        ParamExampleBlendMode = "Blend Mode";
        ParamExampleBlendModeDesc = "How the solid color is blended with the image.";

        // SolidColorBlend menu item labels (shared names, example-specific descriptions)
        MenuNormal = "Normal";
        MenuMultiply = "Multiply";
        MenuScreen = "Screen";
        MenuOverlay = "Overlay";
        MenuExampleNormalDesc = "Linear interpolation between image and solid color.";
        MenuExampleMultiplyDesc = "Multiplies the image by the solid color.";
        MenuExampleScreenDesc = "Screens the image with the solid color (inverse multiply).";
        MenuExampleOverlayDesc = "Combines Multiply and Screen based on image brightness.";

        // Standard / legacy
        ParamColor = "Color";
        ParamColorDesc = "Solid color for the effect.";
        ParamStandardBlendMode = "Blend Mode";
        ParamStandardBlendModeDesc = "How the solid color is blended with the image.";
        ParamGroup1 = "Group1";
        ParamGroup1Desc = "Nested group with inner parameters.";
        ParamInnerFloat = "Inner Float";
        ParamInnerFloatDesc = "A floating-point parameter inside a group.";
        ParamInnerBool = "Inner Bool";
        ParamInnerBoolDesc = "A boolean parameter inside a group.";
        ParamExampleEffectName = "Example Effect";
        ParamGroup1Enabled = "Enabled";

        // standard.rs extras
        ParamBrightness = "Brightness";
        ParamBrightnessDesc = "Overall brightness multiplier.";
        ParamInvertColors = "Invert Colors";
        ParamInvertColorsDesc = "Invert all colors in the image.";
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
