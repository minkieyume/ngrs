use ngrs::guile::*;

pub fn main() {    
    with_guile(|vm| {
        println!("Hello guile from Rust!");
        let args = vec!["Test Guile".to_string(),];
        vm.shell(args);
    });
}
