//! Builds and bundles the After Effects plugin for macOS.
//! Builds two variants — Color Adjustment and Solid Blend — via Cargo features.

use clap::builder::PathBufValueParser;

use crate::util::targets::{MACOS_AARCH64, MACOS_X86_64, TARGETS, Target};
use crate::util::{PathBufExt, StatusExt, workspace_dir};

use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const EFFECTS: &[(&str, &str)] = &[
    (
        "color-adjustment",
        "VideoFXExampleColorAdjustment",
    ),
    (
        "solid-blend",
        "VideoFXExampleSolidBlend",
    ),
];

pub fn command() -> clap::Command {
    clap::Command::new("macos-ae-plugin")
        .about(
            "Builds and bundles both After Effects plugins for macOS (Color Adjustment + Solid Blend).",
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
                .help("The directory that the After Effects plugin bundles will be output to")
                .value_parser(PathBufValueParser::new())
                .default_value(workspace_dir().plus("build").as_os_str().to_owned()),
        )
}

fn build_plugin_for_target(
    target: &Target,
    feature: &str,
    release_mode: bool,
) -> std::io::Result<(PathBuf, PathBuf)> {
    println!(
        "Building AE plugin [{}] for target {}",
        feature,
        target.target_triple
    );

    let mut cargo_args: Vec<_> = vec![
        String::from("build"),
        String::from("--package=video-fx-ae-plugin"),
        String::from("--features"),
        String::from(feature),
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

    // Copy to unique name so subsequent feature builds don't overwrite
    let lib_saved = target_dir_path.plus(format!("video_fx_ae_plugin_{feature}"));
    let rsrc_saved = target_dir_path.plus(format!("video-fx-ae-plugin_{feature}.rsrc"));
    if built_library_path.exists() {
        fs::copy(&built_library_path, &lib_saved)?;
    }
    if built_rsrc_path.exists() {
        fs::copy(&built_rsrc_path, &rsrc_saved)?;
    }

    Ok((lib_saved, rsrc_saved))
}

fn create_bundle(
    build_dir: &PathBuf,
    bundle_name: &str,
    lib_path: &PathBuf,
    rsrc_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let plugin_dir = build_dir.plus(format!("{bundle_name}.plugin"));

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
        plist::Value::from(format!("com.example.afterfx.{bundle_name}")),
    );
    info_plist.insert(
        "CFBundlePackageType".to_string(),
        plist::Value::from("eFKT"),
    );
    info_plist.insert("CFBundleSignature".to_string(), plist::Value::from("FXTC"));

    plist::Value::Dictionary(info_plist).to_file_xml(contents.plus("Info.plist"))?;

    fs::copy(lib_path, macos_dir.plus("VideoFx"))?;
    fs::copy(rsrc_path, resources.plus("VideoFx.rsrc"))?;

    println!("Bundled {bundle_name}.plugin");

    Ok(())
}

pub fn main(args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let release_mode = args.get_flag("release");
    let build_dir = args.get_one::<PathBuf>("destdir").unwrap();

    for &(feature, bundle_name) in EFFECTS {
        let (lib_path, rsrc_path) = if args.get_flag("macos-universal") {
            let (x86_lib, x86_rsrc) =
                build_plugin_for_target(MACOS_X86_64, feature, release_mode)?;
            let (arm_lib, _) =
                build_plugin_for_target(MACOS_AARCH64, feature, release_mode)?;

            let merged = std::env::temp_dir().plus(format!(
                "example-ae-{feature}-{}",
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

            build_plugin_for_target(target, feature, release_mode)?
        };

        create_bundle(build_dir, bundle_name, &lib_path, &rsrc_path)?;
    }

    println!("\nBoth AE plugins built successfully.");
    Ok(())
}
