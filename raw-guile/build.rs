use std::env;
use std::path::PathBuf;

fn main() {
    let guile = pkg_config::Config::new()
        .probe("guile-3.0")
        .expect("Guile not found");

    // 从 include 路径中找 libguile.h
    let header = guile.include_paths
        .iter()
        .find_map(|path| {
            let candidate = path.join("libguile.h");
            if candidate.exists() {
                Some(candidate)
            } else {
                None
            }
        })
        .expect("Cannot find libguile.h in include paths");

    // 构建 bindgen，添加所有 include 路径
    let mut builder = bindgen::Builder::default()
        .header(header.to_str().unwrap());

    for include in &guile.include_paths {
        builder = builder.clang_arg(format!("-I{}", include.display()));
    }

    let bindings = builder
        .use_core()  // 使用 core 而非 std
        .generate_block(true)  // ← 添加这行！生成 unsafe extern 块
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("libguile.rs")
        )
        .unwrap();
}
