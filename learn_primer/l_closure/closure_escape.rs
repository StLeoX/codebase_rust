fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
    let s = "2";
    c.push(Box::new(|| println!("1")));
    c.push(Box::new(move || println!("{}", s)));
    c.push(Box::new(|| println!("3")));
}

fn main() {
    // let mut c: Vec<Box<dyn Fn()>> = Vec::new();
    let mut c = Vec::new();
    boxed_closure(&mut c);
    for f in c {
        f();
    }
}