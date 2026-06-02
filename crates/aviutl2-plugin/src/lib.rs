use anyhow::Result;
use aviutl2::AviUtl2Info;
use aviutl2::filter::{
    FilterConfigItem, FilterPlugin, FilterPluginFlags, FilterPluginTable, FilterProcVideo,
};
use aviutl2::generic::{GenericPlugin, GenericPluginTable, HostAppHandle, SubPlugin};
use video_fx::settings::Settings;
use video_fx::{
    ExampleEffect, ExampleEffectFullSettings, SolidColorBlend, SolidColorBlendFullSettings,
};

#[cfg(gpu_available)]
use aviutl2::filter::{ReadableImageResource, ShaderTargetResource, WritableImageResource};

mod aul2;
mod params;

pub use aul2::{generate_aul2_en, generate_aul2_zh_cn, write_aul2_to};

use params::build_config_items;

#[aviutl2::plugin(GenericPlugin)]
struct VideoFxPlugin {
    example_filter: SubPlugin<ExampleFilter>,
    blend_filter: SubPlugin<BlendFilter>,
}

impl GenericPlugin for VideoFxPlugin {
    fn new(info: AviUtl2Info) -> Result<Self> {
        let _ = aviutl2::tracing_subscriber::fmt()
            .with_max_level(aviutl2::tracing::Level::WARN)
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .try_init();

        video_fx::i18n::set_lang(video_fx::i18n::detect_system_lang());
        let example_filter = SubPlugin::<ExampleFilter>::new_filter_plugin(&info)?;
        let blend_filter = SubPlugin::<BlendFilter>::new_filter_plugin(&info)?;
        Ok(Self {
            example_filter,
            blend_filter,
        })
    }

    fn plugin_info(&self) -> GenericPluginTable {
        GenericPluginTable {
            name: "VideoFX-rs".into(),
            information: "VideoFX-rs multi-effect plugin for AviUtl2".into(),
        }
    }

    fn register(&mut self, registry: &mut HostAppHandle) {
        registry.register_filter_plugin(&self.example_filter);
        registry.register_filter_plugin(&self.blend_filter);
    }
}

#[aviutl2::plugin(FilterPlugin)]
struct ExampleFilter;

impl FilterPlugin for ExampleFilter {
    fn new(_info: AviUtl2Info) -> Result<Self> {
        Ok(Self)
    }

    fn plugin_info(&self) -> FilterPluginTable {
        FilterPluginTable {
            name: "VideoFX Example Effect".into(),
            label: Some("VideoFX".into()),
            information: "Brightness, tint, contrast, saturation adjustments.".into(),
            flags: aviutl2::bitflag!(FilterPluginFlags {
                video: true,
                filter: true,
            }),
            config_items: build_config_items::<ExampleEffectFullSettings>(),
        }
    }

    fn proc_video(&self, config: &[FilterConfigItem], video: &mut FilterProcVideo) -> Result<()> {
        let mut settings = ExampleEffectFullSettings::default();
        read_config(config, &mut settings);
        let effect: ExampleEffect = settings.into();

        // GPU compute shader path (unorm float4 → confirmed format)
        #[cfg(gpu_available)]
        {
            match try_gpu_render(&effect, video) {
                Ok(()) => {
                    use std::sync::atomic::{AtomicBool, Ordering};
                    static GPU_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !GPU_LOGGED.swap(true, Ordering::Relaxed) {
                        aviutl2::lprintln!(info, "VideoFX GPU compute shader active");
                    }
                    return Ok(());
                }
                Err(e) => {
                    aviutl2::lprintln!(warn, "GPU render failed: {}, falling back to CPU", e);
                }
            }
        }

        let w = video.video_object.width as usize;
        let h = video.video_object.height as usize;
        if w == 0 || h == 0 {
            return Ok(());
        }
        let len = w * h * 4;
        let mut src = vec![0u8; len];
        let mut dst = vec![0u8; len];
        video.get_image_data(&mut src);
        effect.apply_effect(&src, &mut dst, w, h);
        video.set_image_data(&dst, video.video_object.width, video.video_object.height);
        Ok(())
    }
}

#[cfg(gpu_available)]
fn try_gpu_render(effect: &ExampleEffect, video: &mut FilterProcVideo) -> Result<()> {
    use std::mem::size_of;

    let w = video.video_object.width;
    let h = video.video_object.height;
    if w == 0 || h == 0 {
        return Ok(());
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Uniforms {
        width: u32,
        height: u32,
        brightness: f32,
        tint_r: f32,
        tint_g: f32,
        tint_b: f32,
        invert: u32,
        contrast: f32,
        saturation: f32,
        color_preset: u32,
        _pad: [u32; 2],
    }

    const _: () = assert!(size_of::<Uniforms>() == 48);

    let contrast = effect
        .advanced
        .as_ref()
        .map(|a| a.contrast.clamp(0.0, 4.0))
        .unwrap_or(1.0);
    let saturation = effect
        .advanced
        .as_ref()
        .map(|a| a.saturation.clamp(0.0, 2.0))
        .unwrap_or(1.0);

    let uniforms = Uniforms {
        width: w,
        height: h,
        brightness: effect.brightness.clamp(0.0, 2.0),
        tint_r: effect.tint_r.clamp(0.0, 2.0),
        tint_g: effect.tint_g.clamp(0.0, 2.0),
        tint_b: effect.tint_b.clamp(0.0, 2.0),
        invert: if effect.invert_colors { 1 } else { 0 },
        contrast,
        saturation,
        color_preset: effect.color_preset as u32,
        _pad: [0; 2],
    };

    let cso_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/standard.cso"));
    let thread_x = (w + 15) / 16;
    let thread_y = (h + 15) / 16;

    // GPU-to-GPU copy: Object → named resource for SRV access.
    // Object is not directly readable as SRV in compute shaders,
    // so we copy it to a named resource that IS SRV-accessible.
    video.copy_image_resource(
        &ReadableImageResource::Object,
        &WritableImageResource::Resource("video_fx_input".into()),
    )?;

    video.exec_computeshader_data(
        cso_bytes,
        &[ShaderTargetResource::Object],
        &[ReadableImageResource::Resource("video_fx_input".into())],
        uniforms,
        [thread_x, thread_y, 1],
        None,
    )?;

    Ok(())
}

#[aviutl2::plugin(FilterPlugin)]
struct BlendFilter;

impl FilterPlugin for BlendFilter {
    fn new(_info: AviUtl2Info) -> Result<Self> {
        Ok(Self)
    }

    fn plugin_info(&self) -> FilterPluginTable {
        FilterPluginTable {
            name: "VideoFX Solid Color Blend".into(),
            label: Some("VideoFX".into()),
            information: "Blend a solid color with configurable blend modes.".into(),
            flags: aviutl2::bitflag!(FilterPluginFlags {
                video: true,
                filter: true,
            }),
            config_items: build_config_items::<SolidColorBlendFullSettings>(),
        }
    }

    fn proc_video(&self, config: &[FilterConfigItem], video: &mut FilterProcVideo) -> Result<()> {
        let mut settings = SolidColorBlendFullSettings::default();
        read_config(config, &mut settings);
        let effect: SolidColorBlend = settings.into();

        let w = video.video_object.width as usize;
        let h = video.video_object.height as usize;
        if w == 0 || h == 0 {
            return Ok(());
        }
        let len = w * h * 4;
        let mut src = vec![0u8; len];
        let mut dst = vec![0u8; len];
        video.get_image_data(&mut src);
        effect.apply_effect(&src, &mut dst, w, h);
        video.set_image_data(&dst, video.video_object.width, video.video_object.height);
        Ok(())
    }
}

fn read_config<T: Settings>(config: &[FilterConfigItem], settings: &mut T) {
    let descriptors = T::setting_descriptors();
    let mut idx = 0;
    read_descriptors(&descriptors, config, settings, &mut idx);
}

fn read_descriptors<T: Settings>(
    descriptors: &[video_fx::settings::SettingDescriptor<T>],
    config: &[FilterConfigItem],
    settings: &mut T,
    idx: &mut usize,
) {
    use video_fx::settings::{EnumValue, SettingKind};

    for desc in descriptors {
        match &desc.kind {
            SettingKind::FloatRange { .. } | SettingKind::Percentage { .. } => {
                if let Some(FilterConfigItem::Track(track)) = config.get(*idx) {
                    let _ = settings.set_field::<f32>(&desc.id, track.value as f32);
                }
                *idx += 1;
            }
            SettingKind::IntRange { .. } => {
                if let Some(FilterConfigItem::Track(track)) = config.get(*idx) {
                    let _ = settings.set_field::<i32>(&desc.id, track.value as i32);
                }
                *idx += 1;
            }
            SettingKind::Boolean => {
                if let Some(FilterConfigItem::Checkbox(check)) = config.get(*idx) {
                    let _ = settings.set_field::<bool>(&desc.id, check.value);
                }
                *idx += 1;
            }
            SettingKind::Enumeration { .. } => {
                if let Some(FilterConfigItem::Select(select)) = config.get(*idx)
                    && let Some(item) = select.items.get(select.value as usize)
                {
                    let enum_val = EnumValue(item.value as u32);
                    let _ = settings.set_field::<EnumValue>(&desc.id, enum_val);
                }
                *idx += 1;
            }
            SettingKind::Group { children } => {
                match config.get(*idx) {
                    Some(FilterConfigItem::Checkbox(check)) => {
                        let _ = settings.set_field::<bool>(&desc.id, check.value);
                    }
                    Some(FilterConfigItem::CheckSection(check)) => {
                        let _ = settings.set_field::<bool>(&desc.id, check.value);
                    }
                    _ => {}
                }
                *idx += 1;

                if let Some(FilterConfigItem::Group(_)) = config.get(*idx) {
                    *idx += 1;
                }

                read_descriptors(children, config, settings, idx);

                if let Some(FilterConfigItem::Group(g)) = config.get(*idx)
                    && g.name.is_none()
                {
                    *idx += 1;
                }
            }
        }
    }
}

aviutl2::register_generic_plugin!(VideoFxPlugin);
