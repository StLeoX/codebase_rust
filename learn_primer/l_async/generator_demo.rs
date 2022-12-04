#![feature(generators, generator_trait)]
use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let mut generator = || {  // 生成器的定义，类似于无参闭包。
        yield 1;
        yield 2;
        yield 3;
        return 4;
    };

    for _ in 0..4 {
        let r = Pin::new(&mut generator).resume(());
        println!("{:?}", r);
    }
}


// Generator Trait 概要设计
pub trait Generator {
    type Yield;
    type Return;
    unsafe fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

// resume -> GeneratorState
pub enum GeneratorState {
    Yielded;
    Complete;
}

// Yielder 概要设计
// 注意此处Yielder迭代器并不是指Iterator
type Yielder<Y> = Generator<Yield = Y, Return = ()>;

// Future 概要设计
type Future<T, E> = Generator<Yield = (), Return = Result<T, E>>;

