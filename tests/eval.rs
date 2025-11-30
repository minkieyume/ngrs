// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use guile::*;
use std::sync::Once;

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
