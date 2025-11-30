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
fn use_with_guile() {
    with_guile(|_| {
        println!("Hello guile from Rust!");
        without_guile(|| {
            println!("Hello guile from Rust!");
        });
    });
}
    
#[test]
fn can_init_twice() {
    let _runtime = Runtime::new();
    let _runtime2 = Runtime::new();
    assert!(true, "Initialized twice successfully");
}
