// 在此模式中，匹配到的重复序列将被直接丢弃，仅留用它所带来的长度信息
// 最常见的用法就是 计数

macro_rules! replace {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

use std::default::Default;
macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace!(
                    ($tup_tys)
                    Default::default()
                ),
            )*
        )
    };
}

#[test]
pub fn replace_main() {
    assert_eq!(tuple_default!(i32, bool, String), (0, false, String::new()));
}

/// 匹配和替换

// 为NewType Struct自动实现new方法，以StructName()的形式。
macro_rules! newtype_new {
    // (@as_item $i:item) => {$i};
    (struct $name:ident($t:ty);) => { newtype_new! { () struct $name($t); } };
    (pub struct $name:ident($t:ty);) => { newtype_new! { (pub) struct $name($t); } };
    (($($vis:tt)*) struct $name:ident($t:ty);) => {
        as_item! {
            impl $name {
                $($vis)* fn new(value: $t) -> Self {
                    $name(value)
                }
            }
        }
    };
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
#[derive(Debug, Eq, PartialEq)]
struct Dummy(i32);

newtype_new! { struct Dummy(i32); }

pub fn main02() {
    println!("{:?}", Dummy(1));
    assert_eq!(Dummy::new(42), Dummy(42));
}
