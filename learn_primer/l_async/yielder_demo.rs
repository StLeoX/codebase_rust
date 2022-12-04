// Yielder 概要设计
// 注意此处Yielder迭代器并不是指Iterator
// type Yielder<Y> = Generator<Yield = Y, Return = ()>;

#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn up_to() -> impl Generator<Yield = u32, Return = ()> {
    || {
        let mut x = 0;
        loop {
            x += 1;
            yield x;
        }
        // return ();
    }
}

fn main() {
    let mut gen = up_to();
    for _ in 0..10 {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(y) => println!("{}", y),
            _ => println!("End"),  // 一般不会命中
        }
    }
}
