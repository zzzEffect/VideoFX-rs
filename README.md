# VideoFX-rs — Rust Video Plugin Framework

A general-purpose framework for building cross-host video effect plugins in Rust. Derived from [ntsc-rs](https://github.com/valadaptive/ntsc-rs), VideoFX-rs provides the infrastructure for creating effects that target After Effects, Premiere, OpenFX hosts (such as VEGAS Pro, DaVinci Resolve), and AviUtl2 (ExEdit2).

## Structure

```
VideoFX-rs/
├── crates/
│   ├── example-effects/          # Core library (settings, i18n, GPU, effects)
│   ├── macros/                   # Proc macro: #[derive(FullSettings)]
│   ├── ae-plugin/                # After Effects / Premiere plugin (cdylib)
│   ├── openfx-plugin/            # OpenFX plugin (cdylib)
│   │   └── vendor/
│   │       └── openfx/           # OpenFX SDK (git submodule)
│   └── aviutl2-plugin/          # AviUtl2 filter plugin (cdylib, Windows only)
└── xtask/                        # Build & bundle helper (cargo xtask)
```

## Supported Hosts

| Host | Example Crate | Build Command |
|------|---------------|---------------|
| After Effects / Premiere | `ae-plugin` | `cargo build -p video-fx-ae-plugin` |
| OpenFX (Resolve, VEGAS, etc.) | `openfx-plugin` | `cargo xtask build-ofx-plugin` |
| AviUtl2 (ExEdit2) | `aviutl2-plugin` | `cargo xtask build-aviutl2-plugin --release` |

## Effects

This project includes two example effects:

| Effect | Description |
|--------|-------------|
| **VideoFX Example Color Adjustment** | Brightness, per-channel tint, invert colors, contrast, saturation, and color presets (Warm/Cool/Sepia). |
| **VideoFX Example Solid Blend** | Solid color overlay with blend modes (Normal, Multiply, Screen, Overlay). |

## Quick Start

### Check compilation

```bash
cargo check --workspace
```

### Run tests

```bash
cargo test --workspace
```

## Building from Source

### Install Rust

Install the latest stable Rust via [rustup](https://rustup.rs/):

```bash
rustup install stable
```

You may need to close and reopen your terminal after this.

### Install rust-bindgen requirements (OpenFX only)

If you want to build the OpenFX plugin, you'll need to install dependencies for the rust-bindgen tool to work.

If you're not building the OpenFX plugin, you can ignore this part.

### Clone the Repository

Make sure to include submodules when cloning the repository if you want the OpenFX plugin to build properly:

```bash
git clone --recurse-submodules https://github.com/zzzEffect/VideoFX-rs.git
cd VideoFX-rs
```

If you've already cloned the repository without submodules, you can initialize them via:

```bash
git submodule update --init --recursive
```

### Platform-specific Instructions

After installing Rust and cloning the repository, the steps are platform-specific:

#### Windows

```bash
# Build the OpenFX plugin (output: `crates/openfx-plugin/build/VideoFx.ofx.bundle`)
# Contains both effects in a single bundle.
cargo xtask build-ofx-plugin --release

# Build the After Effects plugin (both effects in a single .aex)
# The output DLL is `target/release/video_fx_ae_plugin.dll`.
# Rename and copy to:
# C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\VideoFX.aex
cargo build -p video-fx-ae-plugin --release

# Build the AviUtl2 filter plugin (output: `crates/aviutl2-plugin/build/VideoFX-rs.au2pkg.zip`)
cargo xtask build-aviutl2-plugin --release
```

#### macOS

```bash
# Build the OpenFX plugin (output will be in `crates/openfx-plugin/build`)
cargo xtask build-ofx-plugin --macos-universal --release

# Build and bundle the After Effects plugin (output will be in the `build` folder)
cargo xtask macos-ae-plugin --macos-universal --release
```

#### Linux

```bash
# Build the OpenFX plugin (output will be in `crates/openfx-plugin/build`)
cargo xtask build-ofx-plugin --release
```

## How to Write a Plugin

### 1. Define your effect parameters

```rust
use video_fx_macros::FullSettings;

#[derive(FullSettings, Clone, Debug, PartialEq)]
pub struct MyEffect {
    pub brightness: f32,
    pub invert_colors: bool,
    // ...
}
```

### 2. Implement the `Settings` trait with `setting_descriptors()`

This provides introspectable parameter descriptions used by AE, OFX, and AviUtl2 plugin hosts.

### 3. Write the effect render function

```rust
impl MyEffect {
    pub fn apply_effect(&self, src: &[u8], dst: &mut [u8], width: usize, height: usize) {
        // Your effect logic here
    }
}
```

### 4. Plugins automatically map parameters

`ae-plugin`, `openfx-plugin`, and `aviutl2-plugin` use the generic `SettingsList` to:
- Generate host-specific UI controls (sliders, checkboxes, dropdowns)
- Read parameter values back during render
- Support preset load/save (JSON)

## Framework Crates

### example-effects (core)

The core library (`crates/example-effects/`, package `example-effects`) bundles the settings framework, i18n system, GPU device management, and example effects:
- `Settings` trait with `get_field`/`set_field` for type-erased parameter access
- `SettingsList<T>` for introspection, JSON serialization/deserialization
- `I18nKey` trait and `i18n_keys!` macro for translation key generation
- `SettingDescriptor`, `SettingKind`, `MenuItem` for describing parameters to plugin hosts
- `setting_id!` macro for concise parameter ID creation
- `get_or_init_shared_device()` / `is_shared_device_ready()` — shared wgpu device management
- Localization support: English, 简体中文, 日本語, 한국어

### macros

The proc-macro crate (`crates/macros/`, package `video-fx-macros`) provides `#[derive(FullSettings)]` which generates a companion `*FullSettings` struct where all `#[settings_block]` fields become non-optional `SettingsBlock<T>` fields for persistent UI state.

## Testing with Real Hosts

### OpenFX

Copy `crates/openfx-plugin/build/VideoFx.ofx.bundle/` to your OFX host's plugins directory:
- **DaVinci Resolve**: `C:\ProgramData\Blackmagic Design\DaVinci Resolve\Support\OFXPlugins\`
- **Natron**: `C:\Program Files\Common Files\OFX\Plugins\`

The bundle contains both effects under the **"VideoFX Example"** category.

### After Effects / Premiere

Copy the built `.aex` file to:
- `C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\`

The plugins appear under the **"VideoFX Example"** category as **"VideoFX Example Color Adjustment"** and **"VideoFX Example Solid Blend"**.

### AviUtl2

Drag `VideoFX-rs.au2pkg.zip` onto the AviUtl2 preview window, or extract the contents into the AviUtl2 installation directory:

- `Plugin/VideoFX.aux2` — the filter plugin
- `Language/English.video_fx_aviutl2_plugin.aul2` — English labels
- `Language/简体中文.video_fx_aviutl2_plugin.aul2` — Chinese (Simplified) labels
- `Language/Japanese.video_fx_aviutl2_plugin.aul2` — Japanese labels
- `Language/한국어.video_fx_aviutl2_plugin.aul2` — Korean labels

After a restart, the effects appear in ExEdit2's filter list under **"VideoFX Example"** as **"VideoFX Example Color Adjustment"** and **"VideoFX Example Solid Blend"**.

## License

MIT
