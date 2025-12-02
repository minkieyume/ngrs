// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use ngrs::*;
use std::sync::Once;
use std::env;
use std::path::PathBuf;

static INIT: Once = Once::new();

#[ctor::ctor]
fn global_init() {
    INIT.call_once(|| {
        Runtime::initialize();
    });
}

#[test]
fn can_eval_string() {
    let runtime = Runtime::new();
    runtime.eval_string("(+ 1 2 3)");
    assert!(true, "Evaluated successfully");
}

#[test]
fn can_eval_with_guile() {
    with_guile(|vm| {
        vm.eval_string("(+ 1 2 3)");            
    });
}

#[test]
fn can_eval_get_value() {
    with_guile(|vm| {
        let value = vm.eval_string("(+ 1 2 3)");
        println!("Evaluated value: {:?}", value);
    });
}

#[test]
fn can_eval_and_get_number() {
    with_guile(|vm| {
        let value:i32 = vm.eval_string("(+ 1 2 3)").try_into().unwrap();
        assert_eq!(value, 6);
    });
}

#[test]
fn can_eval_and_get_string() {
    with_guile(|vm| {
        let value:String = vm.eval_string("(string-append \"kkp\" \"PPL\")").try_into().unwrap();
        assert_eq!(value, "kkpPPL");
    });
}

#[test]
fn can_load_file() {    
    with_guile(|vm| {
        let current_dir:PathBuf = env::current_dir().unwrap();
        let path = current_dir.join("scm/test-loadfile.scm");
        let filename = path.to_str().unwrap();
        let scm = vm.primitive_load(filename);
        assert_eq!(scm.to_string(), "6");
    });
}
