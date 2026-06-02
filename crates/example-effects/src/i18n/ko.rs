//! Korean (ko) translations for VideoFX Example ExTrKey variants.

use std::ffi::CStr;
use crate::i18n::ExTrKey;

pub fn translate_cstr(key: ExTrKey) -> &'static CStr {
    match key {
        // Effect display names
        ExTrKey::ParamColorAdjustmentName => c"VideoFX Example 색상 조정",
        ExTrKey::ParamSolidBlendName => c"VideoFX Example 솔리드 블렌드",

        // SolidColorBlend param labels
        ExTrKey::ParamColor => c"색상",
        ExTrKey::ParamColorDesc => c"단색과 혼합량 (RGBA).",
        ExTrKey::ParamColorRed => c"색상 - 빨강",
        ExTrKey::ParamColorRedDesc => c"단색의 빨간색 성분입니다.",
        ExTrKey::ParamColorGreen => c"색상 - 초록",
        ExTrKey::ParamColorGreenDesc => c"단색의 초록색 성분입니다.",
        ExTrKey::ParamColorBlue => c"색상 - 파랑",
        ExTrKey::ParamColorBlueDesc => c"단색의 파란색 성분입니다.",
        ExTrKey::ParamBlendAmount => c"혼합량",
        ExTrKey::ParamBlendAmountDesc => c"알파 채널 혼합. 0% = 원본 이미지, 100% = 단색.",
        ExTrKey::ParamBlendAttenuation => c"블렌드 감쇠",
        ExTrKey::ParamBlendAttenuationDesc => c"알파 채널의 블렌드 효과를 감쇠합니다. 100% = 블렌드 없음 (알파 통과), 0% = 완전 블렌드.",
        ExTrKey::ParamBlendMode => c"혼합 모드",
        ExTrKey::ParamBlendModeDesc => c"단색과 이미지의 혼합 방식입니다.",

        // SolidColorBlend menu item labels
        ExTrKey::MenuNormal => c"일반",
        ExTrKey::MenuMultiply => c"곱하기",
        ExTrKey::MenuScreen => c"스크린",
        ExTrKey::MenuOverlay => c"오버레이",
        ExTrKey::MenuNormalDesc => c"이미지와 단색 간의 선형 보간.",
        ExTrKey::MenuMultiplyDesc => c"이미지에 단색을 곱합니다.",
        ExTrKey::MenuScreenDesc => c"단색으로 스크린 효과 (곱하기의 반대).",
        ExTrKey::MenuOverlayDesc => c"이미지 밝기에 따라 곱하기와 스크린을 결합합니다.",

        // ColorAdjustment param labels
        ExTrKey::ParamBrightness => c"밝기",
        ExTrKey::ParamBrightnessDesc => c"전체 밝기 배율입니다.",
        ExTrKey::ParamInvertColors => c"색상 반전",
        ExTrKey::ParamInvertColorsDesc => c"이미지의 모든 색상을 반전합니다.",
        ExTrKey::ParamTint => c"색조",
        ExTrKey::ParamTintDesc => c"채널별 색조 배율 (RGB).",
        ExTrKey::ParamTintRed => c"색조 - 빨강",
        ExTrKey::ParamTintRedDesc => c"빨간색 채널 색조 배율입니다.",
        ExTrKey::ParamTintGreen => c"색조 - 초록",
        ExTrKey::ParamTintGreenDesc => c"초록색 채널 색조 배율입니다.",
        ExTrKey::ParamTintBlue => c"색조 - 파랑",
        ExTrKey::ParamTintBlueDesc => c"파란색 채널 색조 배율입니다.",
        ExTrKey::ParamAdvanced => c"고급 설정",
        ExTrKey::ParamAdvancedDesc => c"추가 고급 설정입니다.",
        ExTrKey::ParamContrast => c"대비",
        ExTrKey::ParamContrastDesc => c"대비 조정입니다.",
        ExTrKey::ParamSaturation => c"채도",
        ExTrKey::ParamSaturationDesc => c"색상 채도 조정입니다.",
        ExTrKey::ParamColorPreset => c"색상 프리셋",
        ExTrKey::ParamColorPresetDesc => c"색상 프리셋을 선택합니다.",
        ExTrKey::MenuNone => c"없음",
        ExTrKey::MenuNoneDesc => c"색상 프리셋 없음.",
        ExTrKey::MenuWarm => c"따뜻한 색",
        ExTrKey::MenuWarmDesc => c"따뜻한 색조.",
        ExTrKey::MenuCool => c"차가운 색",
        ExTrKey::MenuCoolDesc => c"차가운 색조.",
        ExTrKey::MenuSepia => c"세피아",
        ExTrKey::MenuSepiaDesc => c"세피아 색조.",
    }
}
