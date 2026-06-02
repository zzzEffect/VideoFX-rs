use std::io;
use std::path::Path;

/// Generates the English .aul2 file content.
/// Keys are Japanese (the built-in language). Values are English translations.
pub fn generate_aul2_en() -> String {
    let mut out = String::new();
    out.push_str("[VideoFX Example Color Adjustment]\n");
    for (key, val) in COLOR_ADJUSTMENT_EN {
        out.push_str(&format!("{key}={val}\n"));
    }
    out.push('\n');
    out.push_str("[VideoFX Example Solid Blend]\n");
    for (key, val) in SOLID_BLEND_EN {
        out.push_str(&format!("{key}={val}\n"));
    }
    out
}

/// Generates the 简体中文 .aul2 file content.
/// Keys are Japanese (the built-in language). Values are Chinese translations.
pub fn generate_aul2_zh_cn() -> String {
    let mut out = String::new();
    out.push_str("[VideoFX Example Color Adjustment]\n");
    for (key, val) in COLOR_ADJUSTMENT_ZH_CN {
        out.push_str(&format!("{key}={val}\n"));
    }
    out.push('\n');
    out.push_str("[VideoFX Example Solid Blend]\n");
    for (key, val) in SOLID_BLEND_ZH_CN {
        out.push_str(&format!("{key}={val}\n"));
    }
    out
}

/// Generates the 한국어 .aul2 file content.
/// Keys are Japanese (the built-in language). Values are Korean translations.
pub fn generate_aul2_ko() -> String {
    let mut out = String::new();
    out.push_str("[VideoFX Example Color Adjustment]\n");
    for (key, val) in COLOR_ADJUSTMENT_KO {
        out.push_str(&format!("{key}={val}\n"));
    }
    out.push('\n');
    out.push_str("[VideoFX Example Solid Blend]\n");
    for (key, val) in SOLID_BLEND_KO {
        out.push_str(&format!("{key}={val}\n"));
    }
    out
}

pub fn write_aul2_to<P: AsRef<Path>>(dir: P, lang: &str, content: &str) -> io::Result<()> {
    let filename = format!("{lang}.video_fx_aviutl2_plugin.aul2");
    let path = dir.as_ref().join(filename);
    std::fs::write(&path, content)?;
    Ok(())
}

// ---------- .aul2 label tables ----------
// Keys = Japanese (built-in language). Values = target language.

// English translations (key = Japanese, value = English)
const COLOR_ADJUSTMENT_EN: &[(&str, &str)] = &[
    ("明るさ", "Brightness"),
    ("色反転", "Invert Colors"),
    ("色合い", "Tint"),
    ("詳細設定", "Advanced"),
    ("コントラスト", "Contrast"),
    ("彩度", "Saturation"),
    ("カラープリセット", "Color Preset"),
    ("なし", "None"),
    ("暖色", "Warm"),
    ("寒色", "Cool"),
    ("セピア", "Sepia"),
];

const SOLID_BLEND_EN: &[(&str, &str)] = &[
    ("色", "Color"),
    ("ブレンド量", "Blend Amount"),
    ("ブレンド減衰", "Blend Attenuation"),
    ("ブレンドモード", "Blend Mode"),
    ("通常", "Normal"),
    ("乗算", "Multiply"),
    ("スクリーン", "Screen"),
    ("オーバーレイ", "Overlay"),
];

// Chinese (Simplified) translations (key = Japanese, value = 中文)
const COLOR_ADJUSTMENT_ZH_CN: &[(&str, &str)] = &[
    ("明るさ", "亮度"),
    ("色反転", "反转颜色"),
    ("色合い", "色调"),
    ("詳細設定", "高级"),
    ("コントラスト", "对比度"),
    ("彩度", "饱和度"),
    ("カラープリセット", "颜色预设"),
    ("なし", "无"),
    ("暖色", "暖色"),
    ("寒色", "冷色"),
    ("セピア", "怀旧"),
];

const SOLID_BLEND_ZH_CN: &[(&str, &str)] = &[
    ("色", "颜色"),
    ("ブレンド量", "混合量"),
    ("ブレンド減衰", "混合衰减"),
    ("ブレンドモード", "混合模式"),
    ("通常", "正常"),
    ("乗算", "正片叠底"),
    ("スクリーン", "滤色"),
    ("オーバーレイ", "叠加"),
];

// Korean translations (key = Japanese, value = 한국어)
const COLOR_ADJUSTMENT_KO: &[(&str, &str)] = &[
    ("明るさ", "밝기"),
    ("色反転", "색상 반전"),
    ("色合い", "색조"),
    ("詳細設定", "고급 설정"),
    ("コントラスト", "대비"),
    ("彩度", "채도"),
    ("カラープリセット", "색상 프리셋"),
    ("なし", "없음"),
    ("暖色", "따뜻한 색"),
    ("寒色", "차가운 색"),
    ("セピア", "세피아"),
];

const SOLID_BLEND_KO: &[(&str, &str)] = &[
    ("色", "색상"),
    ("ブレンド量", "혼합량"),
    ("ブレンド減衰", "블렌드 감쇠"),
    ("ブレンドモード", "혼합 모드"),
    ("通常", "일반"),
    ("乗算", "곱하기"),
    ("スクリーン", "스크린"),
    ("オーバーレイ", "오버레이"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_english_aul2() {
        let content = generate_aul2_en();
        assert!(content.contains("[VideoFX Example Color Adjustment]"));
        assert!(content.contains("明るさ=Brightness"));
        assert!(content.contains("[VideoFX Example Solid Blend]"));
        assert!(content.contains("通常=Normal"));
    }

    #[test]
    fn generates_chinese_aul2() {
        let content = generate_aul2_zh_cn();
        assert!(content.contains("[VideoFX Example Color Adjustment]"));
        assert!(content.contains("明るさ=亮度"));
        assert!(content.contains("[VideoFX Example Solid Blend]"));
        assert!(content.contains("通常=正常"));
    }

    #[test]
    fn generates_korean_aul2() {
        let content = generate_aul2_ko();
        assert!(content.contains("[VideoFX Example Color Adjustment]"));
        assert!(content.contains("明るさ=밝기"));
        assert!(content.contains("[VideoFX Example Solid Blend]"));
        assert!(content.contains("通常=일반"));
    }
}
