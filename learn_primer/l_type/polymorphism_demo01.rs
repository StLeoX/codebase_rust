fn demo01 {
    let x = "1";
    let x_int :i32 = x.parse().unwrap();  // 针对泛型的类型推导，需要指明to-type
    assert_eq!(x_int, 1);
}


fn demo02 {
    let x = "1";
    let x_int = x.parse::<i32>().unwrap();  // 使用turbofish运算符进行泛型实参化
    assert_eq!(x_int, 1);
}

fn main() {
    demo01();
    demo02();
    println!("ok");
}