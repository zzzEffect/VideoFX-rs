# VideoFX-rs — Rust Video Plugin Framework

A general-purpose framework for building cross-host video effect plugins in Rust. Derived from [ntsc-rs](https://github.com/valadaptive/ntsc-rs), VideoFX-rs provides the infrastructure for creating effects that target After Effects, Premiere, and OpenFX hosts (such as VEGAS Pro, DaVinci Resolve, etc.).

## Structure

```
VideoFX-rs/
├── crates/
│   ├── videofx/                 # Core library (settings, i18n, GPU, effects)
│   ├── macros/                  # Proc macro: #[derive(FullSettings)]
│   ├── ae-plugin/               # After Effects / Premiere plugin (cdylib)
│   └── openfx-plugin/           # OpenFX plugin (cdylib)
│       └── vendor/
│           └── openfx/          # OpenFX SDK (git submodule)
└── xtask/                       # Build & bundle helper (cargo xtask)
```

## Supported Hosts

| Host | Example Crate | Build Command |
|------|---------------|---------------|
| After Effects / Premiere | `ae-plugin` | `cargo build -p video-fx-ae-plugin` |
| OpenFX (Resolve, VEGAS, etc.) | `openfx-plugin` | `cargo xtask build-ofx-plugin` |

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

Build the OpenFX plugin and/or After Effects plugin:

```bash
# Build the OpenFX plugin (the output will be `crates/openfx-plugin/build/VideoFx.ofx.bundle`)
cargo xtask build-ofx-plugin --release

# Build the After Effects plugin (the output will be `target/release/video_fx_ae_plugin.dll`)
# To install it, copy + rename the .dll to:
# C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\VideoFx.aex
cargo build -p video-fx-ae-plugin --release
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

This provides introspectable parameter descriptions used by both AE and OFX plugin hosts.

### 3. Write the effect render function

```rust
impl MyEffect {
    pub fn apply_effect(&self, src: &[u8], dst: &mut [u8], width: usize, height: usize) {
        // Your effect logic here
    }
}
```

### 4. Plugins automatically map parameters

Both `ae-plugin` and `openfx-plugin` use the generic `SettingsList` to:
- Generate host-specific UI controls (sliders, checkboxes, dropdowns)
- Read parameter values back during render
- Support preset load/save (JSON)

## Framework Crates

### videofx (core)

The core library (`crates/videofx/`, package `video-fx`) bundles the settings framework, i18n system, GPU device management, and example effects:
- `Settings` trait with `get_field`/`set_field` for type-erased parameter access
- `SettingsList<T>` for introspection, JSON serialization/deserialization
- `I18nKey` trait and `i18n_keys!` macro for translation key generation
- `SettingDescriptor`, `SettingKind`, `MenuItem` for describing parameters to plugin hosts
- `setting_id!` macro for concise parameter ID creation
- `get_or_init_shared_device()` / `is_shared_device_ready()` — shared wgpu device management

### macros

The proc-macro crate (`crates/macros/`, package `video-fx-macros`) provides `#[derive(FullSettings)]` which generates a companion `*FullSettings` struct where all `#[settings_block]` fields become non-optional `SettingsBlock<T>` fields for persistent UI state.

## Testing with Real Hosts

### OpenFX

Copy `crates/openfx-plugin/build/VideoFx.ofx.bundle/` to your OFX host's plugins directory:
- **DaVinci Resolve**: `C:\ProgramData\Blackmagic Design\DaVinci Resolve\Support\OFXPlugins\`
- **Natron**: `C:\Program Files\Common Files\OFX\Plugins\`

### After Effects / Premiere

Copy the built `.aex` to:
- `C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\`

The plugin appears as **"VideoFx Effect"** under the **"Example"** category.

## License

MIT
