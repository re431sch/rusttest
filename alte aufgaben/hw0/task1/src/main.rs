//! hw0 - task1
//!
//! Introdcution to Rust and Cargo

fn main() {
    //println!("Hello, world!");
    multiple_hello("Ferris", 2);
}
///prints "Hello name" n times
pub fn multiple_hello(name: &str, n: i32) {
    for _ in 0..n {
        println!("Hello, {}!", name);
    }
}
