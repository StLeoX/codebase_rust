// Option的实现是一个枚举体
enum MyOption {
    Some(i32),
    None,
}

fn demo01() {
    let s=Some(1);
    let num = s.unwrap();
    match s{
        Some(n)=> println!("num is : {}",n),
        None=>(),
    };
}

////////////////////////////////
// 内置的Option支持泛型
fn demo02() {
    let s: &Option<String> = &Some("hello world".to_string());
    match s{
        Some(s) => println!("s is : {}",s),
        _ => (),
    };
}