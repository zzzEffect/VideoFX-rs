// Solid color blend compute shader. Straight alpha.
// Supports all 4 blend modes: Normal (0), Multiply (1), Screen (2), Overlay (3).
// Pixels are packed as u32: 0xAA_BB_GG_RR in little-endian host memory.

struct FullUniforms {
    width: u32,
    height: u32,
    blend_mode: u32,
    blend_amount: f32,
    blend_attenuation: f32,
    solid_r: f32,
    solid_g: f32,
    solid_b: f32,
}

@group(0) @binding(0) var<storage, read>       src_pixels: array<u32>;
@group(0) @binding(1) var<uniform>              u: FullUniforms;
@group(0) @binding(2) var<storage, read_write>  dst_pixels: array<u32>;

fn blend_alpha_channel(src_a: f32, a: f32, inv: f32, blend_mode: u32) -> f32 {
    let blended = switch blend_mode {
        0u => src_a * inv + a,
        1u => src_a * inv + src_a * a,
        2u => src_a * inv + (1.0 - (1.0 - src_a) * (1.0 - a)) * a,
        3u => {
            let ov = select(1.0 - 2.0 * (1.0 - src_a) * (1.0 - a), 2.0 * src_a * a, src_a < 0.5);
            src_a * inv + ov * a
        }
        default => src_a,
    };
    return src_a + (blended - src_a) * (1.0 - u.blend_attenuation);
}

@compute @workgroup_size(16, 16, 1)
fn solid_blend_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let y = id.y;

    if x >= u.width || y >= u.height {
        return;
    }

    let i = y * u.width + x;
    let packed = src_pixels[i];

    let sr = byte_to_float((packed >> 0u) & 0xFFu);
    let sg = byte_to_float((packed >> 8u) & 0xFFu);
    let sb = byte_to_float((packed >> 16u) & 0xFFu);
    let sa = byte_to_float((packed >> 24u) & 0xFFu);

    let a = u.blend_amount;
    let inv = 1.0 - a;

    var dr: f32;
    var dg: f32;
    var db: f32;

    switch u.blend_mode {
        case 0u: {
            dr = sr * inv + u.solid_r * a;
            dg = sg * inv + u.solid_g * a;
            db = sb * inv + u.solid_b * a;
        }
        case 1u: {
            dr = sr * inv + sr * u.solid_r * a;
            dg = sg * inv + sg * u.solid_g * a;
            db = sb * inv + sb * u.solid_b * a;
        }
        case 2u: {
            dr = sr * inv + (1.0 - (1.0 - sr) * (1.0 - u.solid_r)) * a;
            dg = sg * inv + (1.0 - (1.0 - sg) * (1.0 - u.solid_g)) * a;
            db = sb * inv + (1.0 - (1.0 - sb) * (1.0 - u.solid_b)) * a;
        }
        case 3u: {
            dr = sr * inv + overlay_channel(sr, u.solid_r) * a;
            dg = sg * inv + overlay_channel(sg, u.solid_g) * a;
            db = sb * inv + overlay_channel(sb, u.solid_b) * a;
        }
        default: {
            dr = sr;
            dg = sg;
            db = sb;
        }
    }

    dr = clamp(dr, 0.0, 1.0);
    dg = clamp(dg, 0.0, 1.0);
    db = clamp(db, 0.0, 1.0);
    let da = clamp(blend_alpha_channel(sa, a, inv, u.blend_mode), 0.0, 1.0);

    let out_r = u32(dr * 255.0 + 0.5);
    let out_g = u32(dg * 255.0 + 0.5);
    let out_b = u32(db * 255.0 + 0.5);
    let out_a = u32(da * 255.0 + 0.5);

    dst_pixels[i] = (out_a << 24u) | (out_b << 16u) | (out_g << 8u) | out_r;
}
