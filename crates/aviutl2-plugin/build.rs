use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gpu_available)");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let hlsl_path = manifest_dir.join("shaders").join("standard.hlsl");

    if !hlsl_path.exists() {
        println!("cargo:warning=standard.hlsl not found, GPU path disabled");
        return;
    }

    let fxc = match find_fxc() {
        Some(path) => path,
        None => {
            println!("cargo:warning=fxc.exe not found (Windows SDK), GPU path disabled");
            return;
        }
    };

    println!("cargo:rerun-if-changed={}", hlsl_path.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cso_path = out_dir.join("standard.cso");

    let status = Command::new(&fxc)
        .arg("/T")
        .arg("cs_5_0")
        .arg("/E")
        .arg("main")
        .arg("/Fo")
        .arg(&cso_path)
        .arg(&hlsl_path)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-cfg=gpu_available");
        }
        Ok(s) => {
            println!(
                "cargo:warning=fxc.exe exited with code {:?}, GPU path disabled",
                s.code()
            );
        }
        Err(e) => {
            println!("cargo:warning=fxc.exe failed: {e}, GPU path disabled");
        }
    }
}

fn find_fxc() -> Option<PathBuf> {
    let kits_root = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
    if kits_root.is_dir()
        && let Ok(entries) = fs::read_dir(kits_root)
    {
        let mut versions: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.path())
            .collect();
        versions.sort();
        for ver in versions.iter().rev() {
            let fxc = ver.join("x64").join("fxc.exe");
            if fxc.exists() {
                return Some(fxc);
            }
        }
    }

    if let Ok(paths) = env::var("PATH") {
        for dir in paths.split(';') {
            let fxc = Path::new(dir).join("fxc.exe");
            if fxc.exists() {
                return Some(fxc);
            }
        }
    }

    None
}
