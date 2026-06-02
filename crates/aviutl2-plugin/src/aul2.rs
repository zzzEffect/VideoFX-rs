use std::io;
use std::path::Path;

/// Generates the English .aul2 file content.
/// Since English is the default (labels come from `ExTrKey::en()`),
/// this file maps the default English names back to themselves.
pub fn generate_aul2_en() -> String {
    let mut out = String::new();
    out.push_str("[VideoFX Example Effect]\n");
    for (key, val) in EXAMPLE_EFFECT_LABELS {
        out.push_str(&format!("{key}={val}\n"));
    }
    out.push('\n');
    out.push_str("[VideoFX Solid Color Blend]\n");
    for (key, val) in SOLID_BLEND_LABELS {
        out.push_str(&format!("{key}={val}\n"));
    }
    out
}

/// Generates the 简体中文 .aul2 file content.
/// Maps each default English label to its Chinese translation.
pub fn generate_aul2_zh_cn() -> String {
    let mut out = String::new();
    out.push_str("[VideoFX Example Effect]\n");
    for (key, val) in EXAMPLE_EFFECT_ZH_CN {
        out.push_str(&format!("{key}={val}\n"));
    }
    out.push('\n');
    out.push_str("[VideoFX Solid Color Blend]\n");
    for (key, val) in SOLID_BLEND_ZH_CN {
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

// ---------- label tables ----------

const EXAMPLE_EFFECT_LABELS: &[(&str, &str)] = &[
    ("Brightness", "Brightness"),
    ("Invert Colors", "Invert Colors"),
    ("Tint Red", "Tint Red"),
    ("Tint Green", "Tint Green"),
    ("Tint Blue", "Tint Blue"),
    ("Advanced", "Advanced"),
    ("Contrast", "Contrast"),
    ("Saturation", "Saturation"),
    ("Color Preset", "Color Preset"),
    ("None", "None"),
    ("Warm", "Warm"),
    ("Cool", "Cool"),
    ("Sepia", "Sepia"),
];

const SOLID_BLEND_LABELS: &[(&str, &str)] = &[
    ("Color Red", "Color Red"),
    ("Color Green", "Color Green"),
    ("Color Blue", "Color Blue"),
    ("Blend Amount", "Blend Amount"),
    ("Blend Mode", "Blend Mode"),
    ("Normal", "Normal"),
    ("Multiply", "Multiply"),
    ("Screen", "Screen"),
    ("Overlay", "Overlay"),
];

const EXAMPLE_EFFECT_ZH_CN: &[(&str, &str)] = &[
    ("Brightness", "亮度"),
    ("Invert Colors", "反转颜色"),
    ("Tint Red", "红色调"),
    ("Tint Green", "绿色调"),
    ("Tint Blue", "蓝色调"),
    ("Advanced", "高级"),
    ("Contrast", "对比度"),
    ("Saturation", "饱和度"),
    ("Color Preset", "颜色预设"),
    ("None", "无"),
    ("Warm", "暖色"),
    ("Cool", "冷色"),
    ("Sepia", "怀旧"),
];

const SOLID_BLEND_ZH_CN: &[(&str, &str)] = &[
    ("Color Red", "红色"),
    ("Color Green", "绿色"),
    ("Color Blue", "蓝色"),
    ("Blend Amount", "混合量"),
    ("Blend Mode", "混合模式"),
    ("Normal", "正常"),
    ("Multiply", "正片叠底"),
    ("Screen", "滤色"),
    ("Overlay", "叠加"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_english_aul2() {
        let content = generate_aul2_en();
        assert!(content.contains("[VideoFX Example Effect]"));
        assert!(content.contains("Brightness=Brightness"));
        assert!(content.contains("[VideoFX Solid Color Blend]"));
        assert!(content.contains("Normal=Normal"));
    }

    #[test]
    fn generates_chinese_aul2() {
        let content = generate_aul2_zh_cn();
        assert!(content.contains("[VideoFX Example Effect]"));
        assert!(content.contains("Brightness=亮度"));
        assert!(content.contains("[VideoFX Solid Color Blend]"));
        assert!(content.contains("Normal=正常"));
    }
}
