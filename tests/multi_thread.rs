// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use ngrs::*;
use std::sync::Once;

static INIT: Once = Once::new();

#[ctor::ctor]
fn global_init() {
    INIT.call_once(|| {
        Runtime::initialize();
    });
}

#[test]
fn test_single_thread_multiple_with_guile() {
    // 在同一个线程（主线程）多次调用
    // English: Call multiple times in the same thread (main thread)
    for i in 0..5 {
        with_guile(|rt| {
            println!("Iteration {}", i);
            rt.eval_string("(+ 1 2 3)");
        });
    }
}

#[test]
fn can_init_multithread() {
    use std::thread;
    
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} starting", i);
                Runtime::new();
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn use_with_guile_multithread() {
    use std::thread;
        
    with_guile(|_| {
        println!("Main thread initialized Guile");
    });
        
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} starting", i);
                with_guile(|_| {
                    println!("Hello guile from Rust!");
                    without_guile(|| {
                        println!("Hello guile from Rust!");
                    });
                });
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

