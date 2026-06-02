mod color_adjustment;
pub mod i18n;
mod solid_blend;
pub mod settings;
#[cfg(feature = "gpu")]
pub mod gpu;

/// Reciprocal of 255.0 — multiply by this instead of dividing by 255.0.
pub const RECIP_255: f32 = 1.0_f32 / 255.0_f32;

pub use settings::color_adjustment::{ColorAdjustment, ColorAdjustmentFullSettings};
pub use settings::solid::{SolidColorBlend, SolidColorBlendFullSettings};
