extern crate cc;
use std::fs;
use std::process::{Command, Stdio};

fn build_win_msvc() {
    cc::Build::new()
        .cpp(true)
        .flag("-Zi")
        .flag("-Gm")
        .flag("-INCREMENTAL")
        .warnings(false)
        .define("WIN32", None)
        .include("./src")
        .include("./src/osg")
        .file("./src/tileset.cpp")
        .file("./src/shp23dtile.cpp")
        .file("./src/osgb23dtile.cpp")
        .compile("3dtile");
    // -------------
    println!("cargo:rustc-link-search=native=./lib");
    // -------------
    println!("cargo:rustc-link-lib=gdal_i");
    println!("cargo:rustc-link-lib=osg");
    println!("cargo:rustc-link-lib=osgDB");
    println!("cargo:rustc-link-lib=osgUtil");

    Command::new("cmd")
        .args(
            &["/C", "xcopy", r#".\bin"#, r#".\target\debug"#, "/y", "/e"],
        )
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
    Command::new("cmd")
        .args(
            &["/C", "xcopy", r#".\bin"#, r#".\target\release"#, "/y", "/e"],
        )
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
}

fn build_win_gun() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .warnings(false)
        .define("WIN32", None)
        .include("./src")
        .include("./src/osg")
        .file("./src/tileset.cpp")
        .file("./src/shp23dtile.cpp")
        .file("./src/osgb23dtile.cpp")
        .compile("3dtile");
    // -------------
    println!("cargo:rustc-link-search=native=./lib");
    // -------------
    println!("cargo:rustc-link-lib=gdal_i");
    println!("cargo:rustc-link-lib=osg");
    println!("cargo:rustc-link-lib=osgDB");
    println!("cargo:rustc-link-lib=osgUtil");
    println!("cargo:rustc-link-lib=OpenThreads");

    Command::new("cmd")
        .args(
            &["/C", "xcopy", r#".\bin"#, r#".\target\debug"#, "/y", "/e"],
        )
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
    Command::new("cmd")
        .args(
            &["/C", "xcopy", r#".\bin"#, r#".\target\release"#, "/y", "/e"],
        )
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
}

fn build_linux_unkonw() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .warnings(false)
        .include("./src")
        .include("./src/osg")
        .file("./src/tileset.cpp")
        .file("./src/osgb23dtile.cpp")
        .compile("3dtile");
    // -------------
    println!("cargo:rustc-link-search=native=./lib");
    // -------------
    //      println!("cargo:rustc-link-lib=gdal");
    println!("cargo:rustc-link-lib=OpenThreads");
    println!("cargo:rustc-link-lib=osg");
    println!("cargo:rustc-link-lib=osgDB");
    println!("cargo:rustc-link-lib=osgUtil");
}

fn main() {
    fs::create_dir_all("target/debug").unwrap();
    fs::create_dir_all("target/release").unwrap();
    use std::env;
    match env::var("TARGET") {
        Ok(val) => {
            match val.as_str() {
                "x86_64-pc-windows-gnu" => build_win_gun(),
                "x86_64-unknown-linux-gnu" => build_linux_unkonw(),
                "x86_64-pc-windows-msvc" => build_win_msvc(),
                &_ => {}
            }
        }
        _ => {}
    }
}
