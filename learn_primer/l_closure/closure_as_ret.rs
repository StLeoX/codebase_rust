// 闭包作为返回值，案例一
// 方式一：使用Box包装trait Object返回闭包，注意使用dyn修饰
fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

fn demo01() {
    let f = counter(2);
    assert_eq!(f(1), 3);
}

// 方式二：使用impl Trait返回闭包，注意使用move关键字
fn counter(i: i32) -> impl Fn(i32) -> i32 {
    move |n: i32| -> i32 {n + i}
}

fn demo02() {
    let f = counter(2);
    assert_eq!(f(1), 3);
}



////////////////////////////////////////////////////////////////
// 闭包作为返回值，案例二
// 针对移动语义应该是使用FnOnce Trait

// 某个不稳定的历史写法是FnBox代替FnOnce
// #![feature(fnbox)]
// use std::boxed::FnBox;
// 不过现在已经稳定写作FnOnce，即闭包调用一次.

// 方式一：使用Box包装trait Object返回闭包，注意使用dyn修饰
fn str_suffix_factory() -> Box<dyn FnOnce(String) -> String> {
    Box::new(move |s: String| -> String {s+"suffix"})
}

fn demo03() {
    let str_suffix = str_suffix_factory();
    println!("{}", str_suffix(String::from("hello ")));
}

// 方式二：使用impl Trait返回闭包，注意使用move关键字
fn str_suffix_factory() -> impl FnOnce(String) -> String {
    move |s: String| s+"suffix" 
}

fn demo04() {
    let str_suffix = str_suffix_factory();
    println!("{}", str_suffix(String::from("hello ")));
}


////////////////////////////////////////////////////////////////
// 调用时：显式指定闭包类型
fn demo05() {
    let v1 = 1;
    let clj : Box<Fn() -> i32> = Box::new(|| v1 + 2 );
    assert_eq!(3, clj());
}

