use std::env;
use std::fs::File;
use std::io::prelude::*;

// Result in File load
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut fd = File::open(filename).unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    // 文件内容处理
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>().unwrap();
    }
    println!("{:?}", sum);
}