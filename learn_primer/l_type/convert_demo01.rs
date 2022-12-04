// 为结构实现From Trait
// From和Into在实现上是关联的，通常实现From，就完成了对Into的实现。

use std::convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // 需要显式指定into的转入类型
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
