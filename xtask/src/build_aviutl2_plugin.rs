use clap::builder::PathBufValueParser;

use crate::util::{PathBufExt, StatusExt, workspace_dir};

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const EN_AUL2: &str = "\
[VideoFX Example Effect]
Brightness=Brightness
Invert Colors=Invert Colors
Tint Red=Tint Red
Tint Green=Tint Green
Tint Blue=Tint Blue
Advanced=Advanced
Contrast=Contrast
Saturation=Saturation
Color Preset=Color Preset
None=None
Warm=Warm
Cool=Cool
Sepia=Sepia

[VideoFX Solid Color Blend]
Color Red=Color Red
Color Green=Color Green
Color Blue=Color Blue
Blend Amount=Blend Amount
Blend Mode=Blend Mode
Normal=Normal
Multiply=Multiply
Screen=Screen
Overlay=Overlay
";

const ZH_AUL2: &str = "\
[VideoFX Example Effect]
Brightness=亮度
Invert Colors=反转颜色
Tint Red=红色调
Tint Green=绿色调
Tint Blue=蓝色调
Advanced=高级
Contrast=对比度
Saturation=饱和度
Color Preset=颜色预设
None=无
Warm=暖色
Cool=冷色
Sepia=怀旧

[VideoFX Solid Color Blend]
Color Red=红色
Color Green=绿色
Color Blue=蓝色
Blend Amount=混合量
Blend Mode=混合模式
Normal=正常
Multiply=正片叠底
Screen=滤色
Overlay=叠加
";

const PACKAGE_TOML: &str = "\
id = \"video-fx-rs.aviutl2-plugin\"
name = \"VideoFX-rs\"
version = \"0.1.0\"
information = \"VideoFX-rs multi-effect plugin for AviUtl2\"
";

pub fn command() -> clap::Command {
    clap::Command::new("build-aviutl2-plugin")
        .about("Builds the AviUtl2 filter plugin (.auf2), generates language files, and packages into .au2pkg.zip.")
        .arg(
            clap::Arg::new("release")
                .long("release")
                .help("Build in release mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("destdir")
                .long("destdir")
                .help("Output directory for the build artifacts")
                .value_parser(PathBufValueParser::new())
                .default_value(
                    workspace_dir()
                        .plus_iter(["crates", "aviutl2-plugin", "build"])
                        .as_os_str()
                        .to_owned(),
                ),
        )
}

pub fn main(args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let release_mode = args.get_flag("release");
    let profile = if release_mode { "release" } else { "debug" };

    let output_dir = args.get_one::<PathBuf>("destdir").unwrap();

    let dll_path = build_plugin(release_mode)?;

    fs::create_dir_all(output_dir.plus_iter(["Language"]))?;

    let auf2_path = output_dir.plus("VideoFX.aux2");
    fs::copy(&dll_path, &auf2_path)?;
    println!("Copied DLL → {}", auf2_path.display());

    let en_path = output_dir.plus_iter(["Language", "English.video_fx_aviutl2_plugin.aul2"]);
    fs::write(&en_path, EN_AUL2)?;
    println!("Written English .aul2");

    let zh_path = output_dir.plus_iter(["Language", "简体中文.video_fx_aviutl2_plugin.aul2"]);
    fs::write(&zh_path, ZH_AUL2)?;
    println!("Written 简体中文 .aul2");

    let pkg_path = output_dir.plus("package.txt");
    fs::write(&pkg_path, PACKAGE_TOML)?;
    println!("Written package.txt");

    let zip_path = output_dir.plus("VideoFX-rs.au2pkg.zip");
    write_au2pkg_zip(&zip_path, &auf2_path, &en_path, &zh_path, &pkg_path)?;
    println!("Packaged zip → {}", zip_path.display());

    println!(
        "\nBuild complete ({profile}). Output: {}",
        output_dir.display()
    );

    Ok(())
}

fn build_plugin(release_mode: bool) -> Result<PathBuf, Box<dyn Error>> {
    let profile = if release_mode { "release" } else { "debug" };
    println!("Building AviUtl2 filter plugin ({profile})...");

    let mut cargo_args = vec![
        "build",
        "--package=video-fx-aviutl2-plugin",
        "--lib",
        "--no-default-features",
    ];
    if release_mode {
        cargo_args.push("--release");
    }

    Command::new("cargo")
        .args(&cargo_args)
        .status()
        .expect_success()?;

    let dll_path =
        workspace_dir()
            .to_path_buf()
            .plus_iter(["target", profile, "video_fx_aviutl2_plugin.dll"]);

    if !dll_path.exists() {
        return Err(format!("Build artifact not found: {}", dll_path.display()).into());
    }

    Ok(dll_path)
}

fn write_au2pkg_zip(
    zip_path: &std::path::Path,
    auf2_path: &std::path::Path,
    en_aul2_path: &std::path::Path,
    zh_aul2_path: &std::path::Path,
    pkg_path: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    add_file_to_zip(
        &mut zip,
        options,
        "Plugin/VideoFX.aux2",
        &fs::read(auf2_path)?,
    )?;
    add_file_to_zip(
        &mut zip,
        options,
        "Language/English.video_fx_aviutl2_plugin.aul2",
        &fs::read(en_aul2_path)?,
    )?;
    add_file_to_zip(
        &mut zip,
        options,
        "Language/简体中文.video_fx_aviutl2_plugin.aul2",
        &fs::read(zh_aul2_path)?,
    )?;
    add_file_to_zip(&mut zip, options, "package.txt", &fs::read(pkg_path)?)?;

    zip.finish()?;
    Ok(())
}

fn add_file_to_zip(
    zip: &mut zip::ZipWriter<std::fs::File>,
    options: zip::write::SimpleFileOptions,
    name: &str,
    data: &[u8],
) -> Result<(), Box<dyn Error>> {
    zip.start_file(name, options)?;
    zip.write_all(data)?;
    Ok(())
}
