// 通过元组测试drop顺序，以及drop check的有效性

use std::fmt;

#[derive(Debug, Clone, Copy)]
enum ReturnState {
    Invalid,
    Valid,
}

#[derive(Debug)]
struct DemoStruct<T: fmt::Debug>(&'static str, T, ReturnState);

impl<T: fmt::Debug> DemoStruct<T> {
    fn new(name: &'static str, t: T) -> Self {
        DemoStruct(name, t, ReturnState::Valid)
    }
}

impl<T: fmt::Debug> Drop for DemoStruct<T> {
    fn drop(&mut self) {
        println!("droping {:?}", self);
        self.2 = ReturnState::Invalid;
    }
}

struct WrappedBox<T> {
    value: Box<T>,
}

impl<T> WrappedBox<T> {
    fn new(t: T) -> Self {
        WrappedBox { value: Box::new(t) }
    }
}

fn demo01() {
    // legal
    let x; let y;
    // let (x, y);

    // illegal
    // let y; let x;
    // let (y, x);

    x = DemoStruct::new("x", 1);
    // y <- x. When init y, must have inited x.
    y = WrappedBox::new(DemoStruct::new("y", &x));
    // Output: firstly drop y, then drop x.
}

fn main() {
    demo01()
}
