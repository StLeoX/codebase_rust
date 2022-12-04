//// trait的定义
// 标准库 Add trait 的定义
pub trait Add <RHS = Self> {  // RHS指定了默认值Self，Self代表实现当前Add trait的具体类型。
    type Output;  // 注意：在实现泛型trait的时候，Output必须指明
    fn add(self, rhs: RHS) -> Self::Output;  // Add trait的核心就在于add函数
}

//// trait的实现
// 实现前先导入
use std::ops::Add;  
// 当然，并不必然使用导入的Add trait，这里需要遵循孤儿规则：
// 规则：如果要实现某个trait，那么该trait的定义和要实现该trait的那个类型的定义，至少有一个要在当前crate中写出来。


// 为u32实现Add trait
impl Add for u32 {  // 指明Self类型就是u32；for后面就是Self类型实值。
    type Output = u32;  // 指明Output类型
    fn add(&self, other: u32) -> u32 { self + other }  // "重载"add函数实现具体逻辑
}

// 为String类型实现Add trait
impl Add<&str> for String {
    type Output = String;
    fn add (mut self, other: &str) -> String {
        self.push_str(other);
        self
    }
}

// 用例
fn main01() {
    let a = "hello ";
    let b = "world";
    let c = a.to_string() + b;  // call String::Add<&str>
    println!("{:?}",c);
}

//// 
// 为&str类型实现Add trait，从而实现字符串字面量相加产生新的String
impl Add<&str> for &str {
    type Output = String;
    fn add (self, other: &str) -> String {
        let ret = String::from(self);
        ret.push_str(other);
        ret    
    }
}

fn main02() {
    println!("{:?}", "hello " + "world");
}