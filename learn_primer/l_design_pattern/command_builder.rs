// Rust标准库中的建造者模式
use std::process::Command;

fn main() {
    Command::new("ls")
    .arg("-l")
    .arg("-a")
    .spawn() // exec
    .expect("failed to ls");
}
