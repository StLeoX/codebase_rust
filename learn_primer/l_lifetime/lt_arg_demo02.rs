// 结构体定义中的生命周期参数

#[derive(Debug)]
struct S<'a> {
    s: &'a str,
}

fn main() {
    let a = String::from("hello world");
    let first = a.split(" ").next().unwrap();
    let s1 = S{ s: first};
    assert_eq!("hello", s1.s);
}



////
// 方法定义中的生命周期参数

// 前提是结构体先要使用生命周期参数
#[derive(Debug)]
struct S<'a> {
    s: &'a str,
}

// 在impl中就要声明参数
impl<'a> S<'a> {
    fn split_first(s: &'a str) -> &'a str {
        s.split(' ').next().expect("err")
    }

    fn new(s: &'a str) -> Self {
        S { s:  S::split_first(s)}
    }
}

fn main() {
    let a = String::from("hello world");
    println!("{:?}", S::new(&a));
}