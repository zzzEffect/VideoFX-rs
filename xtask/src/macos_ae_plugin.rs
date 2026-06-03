//! Builds and bundles the After Effects plugin for macOS.
//! A single binary contains both effects (Color Adjustment + Solid Blend).

use clap::builder::PathBufValueParser;

use crate::util::targets::{MACOS_AARCH64, MACOS_X86_64, TARGETS, Target};
use crate::util::{PathBufExt, StatusExt, workspace_dir};

use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn command() -> clap::Command {
    clap::Command::new("macos-ae-plugin")
        .about(
            "Builds and bundles the After Effects plugin for macOS (contains both Color Adjustment and Solid Blend effects).",
        )
        .arg(
            clap::Arg::new("release")
                .long("release")
                .help("Build the plugin in release mode")
                .conflicts_with("debug")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("debug")
                .long("debug")
                .help("Build the plugin in debug mode")
                .conflicts_with("release")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("target")
                .long("target")
                .help("Set the target triple to compile for")
                .default_value(current_platform::CURRENT_PLATFORM),
        )
        .arg(
            clap::Arg::new("macos-universal")
                .long("macos-universal")
                .help("Build a macOS universal library (x86_64 and aarch64)")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("target"),
        )
        .arg(
            clap::Arg::new("destdir")
                .long("destdir")
                .help("The directory that the After Effects plugin bundle will be output to")
                .value_parser(PathBufValueParser::new())
                .default_value(workspace_dir().plus("build").as_os_str().to_owned()),
        )
}

fn build_plugin_for_target(
    target: &Target,
    release_mode: bool,
) -> std::io::Result<(PathBuf, PathBuf)> {
    println!("Building AE plugin for target {}", target.target_triple);

    let mut cargo_args: Vec<_> = vec![
        String::from("build"),
        String::from("--package=video-fx-ae-plugin"),
        String::from("--target"),
        target.target_triple.to_string(),
    ];
    if release_mode {
        cargo_args.push(String::from("--release"));
    }
    Command::new("cargo")
        .args(&cargo_args)
        .status()
        .expect_success()?;

    let profile = if cargo_args.contains(&String::from("--release")) {
        "release"
    } else {
        "debug"
    };

    let target_dir_path = workspace_dir()
        .to_path_buf()
        .plus_iter(["target", target.target_triple, profile]);

    let mut built_library_path = target_dir_path.clone();
    built_library_path.push(target.library_prefix.to_owned() + "video_fx_ae_plugin");
    built_library_path.set_extension(target.library_extension);

    let built_rsrc_path = target_dir_path.plus("video-fx-ae-plugin.rsrc");

    Ok((built_library_path, built_rsrc_path))
}

pub fn main(args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let release_mode = args.get_flag("release");
    let build_dir = args.get_one::<PathBuf>("destdir").unwrap();

    let bundle_name = "VideoFX.plugin";
    let plugin_dir = build_dir.plus(bundle_name);
    let _ = fs::remove_dir_all(&plugin_dir);

    let contents = plugin_dir.plus("Contents");
    fs::create_dir_all(&contents)?;

    let macos_dir = contents.plus("MacOS");
    fs::create_dir_all(&macos_dir)?;

    let resources = contents.plus("Resources");
    fs::create_dir_all(&resources)?;

    fs::write(contents.plus("PkgInfo"), "eFKTFXTC")?;

    let mut info_plist = plist::dictionary::Dictionary::new();
    info_plist.insert(
        "CFBundleIdentifier".to_string(),
        plist::Value::from("com.example.afterfx"),
    );
    info_plist.insert(
        "CFBundlePackageType".to_string(),
        plist::Value::from("eFKT"),
    );
    info_plist.insert("CFBundleSignature".to_string(), plist::Value::from("FXTC"));
    plist::Value::Dictionary(info_plist).to_file_xml(contents.plus("Info.plist"))?;

    let (lib_path, rsrc_path) = if args.get_flag("macos-universal") {
        let (x86_lib, x86_rsrc) = build_plugin_for_target(MACOS_X86_64, release_mode)?;
        let (arm_lib, _) = build_plugin_for_target(MACOS_AARCH64, release_mode)?;

        let merged = std::env::temp_dir().plus(format!(
            "video-fx-ae-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));

        Command::new("lipo")
            .args(&[
                OsString::from("-create"),
                OsString::from("-output"),
                merged.clone().into(),
                x86_lib.into(),
                arm_lib.into(),
            ])
            .status()
            .expect_success()?;

        (merged, x86_rsrc)
    } else {
        let target_triple = args.get_one::<String>("target").unwrap();
        let target = TARGETS
            .iter()
            .find(|t| t.target_triple == target_triple)
            .unwrap_or_else(|| panic!("Target \"{}\" is not supported", target_triple));

        build_plugin_for_target(target, release_mode)?
    };

    fs::copy(&lib_path, macos_dir.plus("VideoFx"))?;
    fs::copy(&rsrc_path, resources.plus("VideoFx.rsrc"))?;

    println!("\nAE plugin built successfully → {bundle_name}");
    Ok(())
}
