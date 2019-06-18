extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if env::var_os("ICU_SYS_NO_PKG_CONFIG").is_none() {
        if let Ok(lib) = pkg_config::find_library("icu-uc") {
            let mut include_flags = String::new();

            for path in lib.include_paths.iter() {
                match path.to_str() {
                    Some(path) => include_flags.push_str(&format!("-I{} ", path)),
                    None => (),
                }
            }

            let mut libdir_flags = String::new();
            for libdir in lib.link_paths.iter() {
                match libdir.to_str() {
                    Some(path) => libdir_flags.push_str(&format!("-L{} ", path)),
                    None => (),
                }
            }
            println!("cargo:include_flags={}", include_flags);
            println!("cargo:libdir_flags={}", libdir_flags);

            let mut link_flags = String::new();
            for l in lib.libs.iter() {
                link_flags.push_str(&format!("-l{} ", l))
            }
            println!("cargo:link_flags={}", link_flags);
            return;
        }
    }

    let install_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("prefix");
    println!("cargo:install_dir={}", install_dir.display());
    println!(
        "cargo:include_flags=-I{}",
        install_dir.join("include").display()
    );
    println!("cargo:libdir_flags={}", install_dir.join("lib").display());
    println!("cargo:link_flags=l:libicuuc.a");
    println!("cargo:rustc-link-lib=static=icuuc");
    println!(
        "cargo:rustc-link-search=native={}",
        install_dir.join("lib").to_str().unwrap()
    );

    assert!(Command::new("make")
        .args(&["-f", "makefile.cargo", "-j", &env::var("NUM_JOBS").unwrap()])
        .status()
        .unwrap()
        .success());
}
