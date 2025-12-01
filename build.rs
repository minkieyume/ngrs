// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>

use std::env;
use std::path::PathBuf;

fn main() {
    let guile = pkg_config::Config::new()
        .probe("guile-3.0")
        .expect("Guile not found");

    println!("cargo:warning=Guile include paths:");
    for path in &guile.include_paths {
        println!("cargo:warning=  {}", path.display());
    }

    // === 1. 编译 wrapper.c（cc 需要知道路径）===
    let mut build = cc::Build::new();
    build.file("src/wrappers/wrapper.c");
    
    // 给 C 编译器添加 include 路径
    for path in &guile.include_paths {
        build.include(path);  // ← wrapper.c 里的 #include <libguile.h> 靠这个找到
    }

    build.compile("guile_wrapper");

    // === 2. bindgen 生成绑定（clang 也需要知道路径）===
    let clang_args: Vec<String> = guile.include_paths
        .iter()
        .map(|p| format!("-I{}", p.display()))
        .collect();

    
    // 构建 bindgen，添加所有 include 路径
    let mut builder = bindgen::Builder::default()
        .header("src/wrappers/wrapper.h")
        .clang_args(&clang_args);

    for include in &guile.include_paths {
        builder = builder.clang_arg(format!("-I{}", include.display()));
    }

    let bindings = builder
        .use_core()
        .generate_block(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("libguile.rs")
        )
        .unwrap();
}
