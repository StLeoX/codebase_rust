// 闭包的延迟执行性：所有权的移动发生于创建时而非执行时。

// 自由变量为复制语义时
fn demo01() {
    let s = "s";
    let c = || { println!("{:?}", s) };
    c();
    c();
    println!("{:?}", s);
}

// 自由变量为移动语义时
fn demo02() {
    let s = "s".to_string();
    let c = move || { println!("{:?}", s) };
    c();
    // c();  // error: use of moved value
    // println!("{:?}", s);  // error: use of moved value
}

// 自由变量为复制语义时，使用move关键字
fn demo03 {
    let s = "s";
    let c = move || { println!("{:?}", s) };
    c();
    c();
    println!("{:?}", s);
}

// 自由变量为移动语义时，使用move关键字
fn demo04 {
    let s = "s".to_string();
    let c = move || { println!("{:?}", s) };
    c();
    c();
    // println!("{:?}", s);  // error: use of moved value
}
