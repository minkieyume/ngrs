extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 方法 1: 使用 pkg-config crate（最干净）
    let guile = pkg_config::Config::new()
        .probe("guile-3.0")  // 或 guile-2.2
        .expect("Failed to find Guile via pkg-config");

    // 动态链接支持
    // #[cfg(feature = "dynamic")]
    // println!("cargo:rustc-link-arg=-Wl,--export-dynamic");

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header_wrapper_or_detect(&guile)  // 自动从 pkg-config 获取路径
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libguile.rs"))
        .expect("Couldn't write bindings!");
}
