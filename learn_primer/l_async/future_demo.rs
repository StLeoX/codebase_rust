// Future的模拟实现
// Future 概要设计
// type Future<T, E> = Generator<Yield = (), Return = Result<T, E>>;

#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

// 这个demo的实际意义并不大。
fn up_to(n: u32) -> impl Generator<Yield = (), Return = Result<u32, ()>> {
    move || {
        for _ in 0..n {
            yield ();
        }
        return Ok(n);
    }
}

fn main() {
    let a = 3;
    let mut gen = up_to(a);
    // 注意此处的加一
    for i in 0..=a {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(x) => println!("count to {}, gen.resume: {:?}", i, x),
            GeneratorState::Complete(the_future) => println!("ready, gen.resume: {:?}", the_future),
        }
    }
}

// 结果：
// count to 0, gen.resume: ()
// count to 1, gen.resume: ()
// count to 2, gen.resume: ()
// ready, gen.resume: Ok(3)
