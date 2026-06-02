// VideoFX Example Effect compute shader (AviUtl2 / D3D11)
// Logic matches crates/videofx/shaders/standard.wgsl exactly.
//
// Binding model (AviUtl2 exec_computeshader_data):
//   resources[0]  = ReadableImageResource::Object  → register(t0) SRV
//   targets[0]    = ShaderTargetResource::Object   → register(u0) UAV
//   constant: T   = cbuffer at register(b0)
//
// Input/Output: R8G8B8A8_UNORM → Texture2D<float4> / RWTexture2D<unorm float4>
// Cbuffer: 48 bytes (16-byte aligned)
// Thread group dispatch: ceil(w/16) × ceil(h/16) × 1

cbuffer Constants : register(b0)
{
    uint width;
    uint height;
    float brightness;
    float tint_r;
    float tint_g;
    float tint_b;
    uint invert;
    float contrast;
    float saturation;
    uint color_preset;
    uint _pad0;
    uint _pad1;
}

Texture2D<float4> Input : register(t0);
RWTexture2D<unorm float4> Output : register(u0);

float luminance(float3 rgb)
{
    return 0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b;
}

[numthreads(16, 16, 1)]
void main(uint3 id : SV_DispatchThreadID)
{
    if (id.x >= width || id.y >= height)
        return;

    float4 src = Input.Load(uint3(id.x, id.y, 0));
    float r = src.r;
    float g = src.g;
    float b = src.b;
    float a = src.a;

    // Brightness
    r *= brightness;
    g *= brightness;
    b *= brightness;

    // Tint (per-channel)
    r *= tint_r;
    g *= tint_g;
    b *= tint_b;

    // Invert
    if (invert != 0) {
        r = 1.0 - r;
        g = 1.0 - g;
        b = 1.0 - b;
    }

    // Contrast (linear around 0.5)
    if (abs(contrast - 1.0) > 0.001) {
        float c = contrast;
        r = (r - 0.5) * c + 0.5;
        g = (g - 0.5) * c + 0.5;
        b = (b - 0.5) * c + 0.5;
    }

    // Saturation (luminance-preserving)
    if (abs(saturation - 1.0) > 0.001) {
        float3 rgb = float3(r, g, b);
        float lum = luminance(rgb);
        float s = saturation;
        r = lum + (r - lum) * s;
        g = lum + (g - lum) * s;
        b = lum + (b - lum) * s;
    }

    // Color preset
    switch (color_preset) {
        case 1: // Warm
            r = r * 1.15;
            g = g * 0.95;
            b = b * 0.75;
            break;
        case 2: // Cool
            r = r * 0.85;
            g = g * 0.95;
            b = b * 1.15;
            break;
        case 3: { // Sepia
            float lr = r, lg = g, lb = b;
            r = lr * 0.393 + lg * 0.769 + lb * 0.189;
            g = lr * 0.349 + lg * 0.686 + lb * 0.168;
            b = lr * 0.272 + lg * 0.534 + lb * 0.131;
            break;
        }
    }

    r = clamp(r, 0.0, 1.0);
    g = clamp(g, 0.0, 1.0);
    b = clamp(b, 0.0, 1.0);

    Output[id.xy] = float4(r, g, b, a);
}
