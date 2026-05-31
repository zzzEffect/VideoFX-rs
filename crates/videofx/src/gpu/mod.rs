pub mod device;
pub mod solid_blend;
pub mod standard;

use std::borrow::Cow;

pub use crate::gpu::device::{get_or_init_shared_device, is_shared_device_ready};

/// Load a WGSL shader by prepending the shared function definitions.
pub(crate) fn load_shader(specific: &'static str) -> wgpu::ShaderSource<'static> {
    let shared = include_str!("../../shaders/shared.wgsl");
    wgpu::ShaderSource::Wgsl(Cow::Owned(format!("{shared}\n{specific}")))
}
