use aviutl2::filter::{
    FilterConfigCheckSection, FilterConfigCheckbox, FilterConfigGroup, FilterConfigItem,
    FilterConfigSelect, FilterConfigSelectItem, FilterConfigTrack,
};
use video_fx::i18n::I18nKey;
use video_fx::settings::{EnumValue, SettingDescriptor, SettingKind, Settings};

pub fn build_config_items<T: Settings + Clone>() -> Vec<FilterConfigItem> {
    let descriptors = T::setting_descriptors();
    let defaults = T::default();
    let mut items = Vec::new();
    for descriptor in descriptors.iter() {
        add_descriptor(descriptor, &defaults, &mut items);
    }
    items
}

fn add_descriptor<T: Settings + Clone>(
    descriptor: &SettingDescriptor<T>,
    defaults: &T,
    items: &mut Vec<FilterConfigItem>,
) {
    let label = descriptor.label_key.en().to_string();

    match &descriptor.kind {
        SettingKind::FloatRange { range, .. } => {
            let min = *range.start() as f64;
            let max = *range.end() as f64;
            let value = defaults.get_field::<f32>(&descriptor.id).unwrap_or(0.0) as f64;
            let step = infer_step(min, max);
            items.push(FilterConfigItem::Track(FilterConfigTrack {
                name: label,
                value,
                range: min..=max,
                step,
                zero_display: None,
                slider_ratio: 1.0,
            }));
        }
        SettingKind::Percentage { .. } => {
            let value = defaults.get_field::<f32>(&descriptor.id).unwrap_or(0.0) as f64;
            items.push(FilterConfigItem::Track(FilterConfigTrack {
                name: label,
                value,
                range: 0.0..=1.0,
                step: 0.01,
                zero_display: None,
                slider_ratio: 1.0,
            }));
        }
        SettingKind::IntRange { range } => {
            let min = *range.start() as f64;
            let max = *range.end() as f64;
            let value = defaults.get_field::<i32>(&descriptor.id).unwrap_or(0) as f64;
            items.push(FilterConfigItem::Track(FilterConfigTrack {
                name: label,
                value,
                range: min..=max,
                step: 1.0,
                zero_display: None,
                slider_ratio: 1.0,
            }));
        }
        SettingKind::Boolean => {
            let value = defaults.get_field::<bool>(&descriptor.id).unwrap_or(false);
            items.push(FilterConfigItem::Checkbox(FilterConfigCheckbox {
                name: label,
                value,
            }));
        }
        SettingKind::Enumeration { options } => {
            let select_items: Vec<FilterConfigSelectItem> = options
                .iter()
                .map(|option| FilterConfigSelectItem {
                    name: option.label_key.en().to_string(),
                    value: option.index as i32,
                })
                .collect();

            let default_enum = defaults
                .get_field::<EnumValue>(&descriptor.id)
                .unwrap_or(EnumValue(0));
            let default_idx = options
                .iter()
                .position(|o| o.index == default_enum.0)
                .unwrap_or(0) as i32;

            items.push(FilterConfigItem::Select(FilterConfigSelect {
                name: label,
                value: default_idx,
                items: select_items,
            }));
        }
        SettingKind::Group { children } => {
            let value = defaults.get_field::<bool>(&descriptor.id).unwrap_or(false);
            items.push(FilterConfigItem::CheckSection(FilterConfigCheckSection {
                name: label.clone(),
                value,
            }));

            items.push(FilterConfigItem::Group(FilterConfigGroup {
                name: Some(label),
                opened: true,
            }));

            for child in children {
                add_descriptor(child, defaults, items);
            }

            items.push(FilterConfigItem::Group(FilterConfigGroup {
                name: None,
                opened: false,
            }));
        }
    }
}

fn infer_step(min: f64, max: f64) -> f64 {
    let range = max - min;
    if range <= 1.0 {
        0.01
    } else if range <= 10.0 {
        0.01
    } else if range <= 100.0 {
        0.1
    } else if range <= 1000.0 {
        1.0
    } else {
        10.0
    }
}
