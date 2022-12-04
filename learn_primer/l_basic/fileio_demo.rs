use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn open_file() {
    let mut file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read to string");
    println!("{}", contents)
    // assert_eq!(contents, "Hello, world!");
}


fn read_file_lines() {
    let file_count = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
    let f_count = BufReader::new(file_count);
    println!("{}", f_count.lines().count());
    
    let file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
    let f = BufReader::new(file);
    for line in f.lines() {
        println!("ugh {}", line.unwrap());
    }
}
