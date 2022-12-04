use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {println!("normally!"); });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {panic!("panic!"); });
    assert!(result.is_err());
    println!("ok");
}