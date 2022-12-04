// 要点1：已实现 Add(&str) for String
// 要点2：已实现 Deref<Target=str>
fn demo01() {
    let a = String::from("a");
    let b = String::from("b");
    let c = a + &b;
    println!("{:?}", c);
}


fn demo02() {
    let a = "a";
    let b: String = a.into();  // 使用&str类型的into方法，转为String类型，基础是已经实现了String::from(s: &str)
}