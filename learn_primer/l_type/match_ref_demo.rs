fn demo01() {
    let x = String::from("a");
    match &*x {
        "a" => println!("a"),
        _ => {}
    }
}