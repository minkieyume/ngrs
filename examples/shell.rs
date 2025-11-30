use guile::*;

pub fn main() {
    let runtime = Runtime::new();
    runtime.with_guile(|vm| {
        println!("Hello guile from Rust!");
        let args = vec!["Test Guile".to_string(),];
        vm.shell(args);
    });
}
