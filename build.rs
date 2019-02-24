extern crate build_tools;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(windows) {
        get_win_sfml_libs();
        get_win_csfml_libs();

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        println!(
            "cargo:rustc-link-search=native={}",
            out_path.to_str().unwrap()
        );
    }
}

fn get_win_sfml_libs() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let sfml_version = String::from("2.5.1");
    let sfml_filename = String::from(format!(
        "SFML-{}-windows-vc15-{}-bit.zip",
        sfml_version,
        get_arch_bit()
    ));
    let sfml_url = String::from(format!("https://www.sfml-dev.org/files/{}", sfml_filename));

    let out_filename = out_dir.join(sfml_filename);

    if !out_filename.exists() {
        build_tools::download_to(dbg!(&sfml_url), out_filename.to_str().unwrap());
    }

    let libs = [
        "flac.lib",
        "freetype.lib",
        "ogg.lib",
        "openal32.lib",
        "sfml-audio.lib",
        "sfml-graphics.lib",
        "sfml-main.lib",
        "sfml-network.lib",
        "sfml-system.lib",
        "sfml-window.lib",
        "vorbis.lib",
        "vorbisenc.lib",
        "vorbisfile.lib",
    ]
    .iter()
    .map(|l| String::from(format!("SFML-{}/lib/{}", sfml_version, l)));

    let dlls = [
        "openal32.dll",
        "sfml-audio-2.dll",
        "sfml-graphics-2.dll",
        "sfml-network-2.dll",
        "sfml-system-2.dll",
        "sfml-window-2.dll",
    ]
    .iter()
    .map(|d| String::from(format!("SFML-{}/bin/{}", sfml_version, d)));

    build_tools::unzip_win_build_files(&out_filename, libs).unwrap();
    build_tools::unzip_win_build_files(&out_filename, dlls).unwrap();
}

fn get_win_csfml_libs() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let sfml_version = String::from("2.5");
    let sfml_filename = String::from(format!(
        "CSFML-{}-windows-{}-bit.zip",
        sfml_version,
        get_arch_bit()
    ));
    let sfml_url = String::from(format!("https://www.sfml-dev.org/files/{}", sfml_filename));

    let out_filename = out_dir.join(sfml_filename);

    if !out_filename.exists() {
        build_tools::download_to(dbg!(&sfml_url), out_filename.to_str().unwrap());
    }

    let libs = [
        "csfml-audio.lib",
        "csfml-graphics.lib",
        "csfml-network.lib",
        "csfml-system.lib",
        "csfml-window.lib",
    ]
    .iter()
    .map(|l| String::from(format!("lib/msvc/{}", l)));

    let dlls = [
        "csfml-audio-2.dll",
        "csfml-graphics-2.dll",
        "csfml-network-2.dll",
        "csfml-system-2.dll",
        "csfml-window-2.dll",
    ]
    .iter()
    .map(|d| String::from(format!("bin/{}", d)));

    build_tools::unzip_win_build_files(&out_filename, libs).unwrap();
    build_tools::unzip_win_build_files(&out_filename, dlls).unwrap();
}

#[cfg(target_pointer_width = "32")]
fn get_arch_bit() -> String {
    "32".to_string()
}

#[cfg(target_pointer_width = "64")]
fn get_arch_bit() -> String {
    "64".to_string()
}
