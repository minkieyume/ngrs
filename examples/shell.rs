// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use ngrs::guile::*;

pub fn main() {    
    with_guile(|vm| {
        println!("Hello guile from Rust!");
        let args = vec!["Test Guile".to_string(),];
        vm.shell(args);
    });
}
