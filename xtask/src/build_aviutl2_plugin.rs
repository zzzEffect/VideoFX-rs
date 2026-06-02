use clap::builder::PathBufValueParser;

use crate::util::{PathBufExt, StatusExt, workspace_dir};

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

// Keys are Japanese (AviUtl2 built-in language). Values are the target language.

const EN_AUL2: &str = "\
[VideoFX Example Color Adjustment]
明るさ=Brightness
色反転=Invert Colors
色合い=Tint
詳細設定=Advanced
コントラスト=Contrast
彩度=Saturation
カラープリセット=Color Preset
なし=None
暖色=Warm
寒色=Cool
セピア=Sepia

[VideoFX Example Solid Blend]
色=Color
ブレンド量=Blend Amount
ブレンド減衰=Blend Attenuation
ブレンドモード=Blend Mode
通常=Normal
乗算=Multiply
スクリーン=Screen
オーバーレイ=Overlay
";

const ZH_AUL2: &str = "\
[VideoFX Example Color Adjustment]
明るさ=亮度
色反転=反转颜色
色合い=色调
詳細設定=高级
コントラスト=对比度
彩度=饱和度
カラープリセット=颜色预设
なし=无
暖色=暖色
寒色=冷色
セピア=怀旧

[VideoFX Example Solid Blend]
色=颜色
ブレンド量=混合量
ブレンド減衰=混合衰减
ブレンドモード=混合模式
通常=正常
乗算=正片叠底
スクリーン=滤色
オーバーレイ=叠加
";

const KO_AUL2: &str = "\
[VideoFX Example Color Adjustment]
明るさ=밝기
色反転=색상 반전
色合い=색조
詳細設定=고급 설정
コントラスト=대비
彩度=채도
カラープリセット=색상 프리셋
なし=없음
暖色=따뜻한 색
寒色=차가운 색
セピア=세피아

[VideoFX Example Solid Blend]
色=색상
ブレンド量=혼합량
ブレンド減衰=블렌드 감쇠
ブレンドモード=혼합 모드
通常=일반
乗算=곱하기
スクリーン=스크린
オーバーレイ=오버레이
";

const PACKAGE_TOML: &str = "\
id = \"video-fx-rs.aviutl2-plugin\"
name = \"VideoFX-rs\"
version = \"0.1.0\"
information = \"VideoFX-rs multi-effect plugin for AviUtl2\"
";

pub fn command() -> clap::Command {
    clap::Command::new("build-aviutl2-plugin")
        .about("Builds the AviUtl2 filter plugin (.aux2), generates language files, and packages into .au2pkg.zip.")
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

    let ko_path = output_dir.plus_iter(["Language", "한국어.video_fx_aviutl2_plugin.aul2"]);
    fs::write(&ko_path, KO_AUL2)?;
    println!("Written 한국어 .aul2");

    let pkg_path = output_dir.plus("package.txt");
    fs::write(&pkg_path, PACKAGE_TOML)?;
    println!("Written package.txt");

    let zip_path = output_dir.plus("VideoFX-rs.au2pkg.zip");
    write_au2pkg_zip(
        &zip_path,
        &auf2_path,
        &en_path,
        &zh_path,
        &ko_path,
        &pkg_path,
    )?;
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
    ko_aul2_path: &std::path::Path,
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
    add_file_to_zip(
        &mut zip,
        options,
        "Language/한국어.video_fx_aviutl2_plugin.aul2",
        &fs::read(ko_aul2_path)?,
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
