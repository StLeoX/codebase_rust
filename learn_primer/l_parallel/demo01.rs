extern crate rayon;
use rayon::prelude::*;

// return-fn
fn sum_of_square(input: &[i32]) -> i32 {
    input.par_iter().map(|&x| x * x).sum()
}

// inplace-fn
fn inc_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|x| *x += 1);
}

pub fn demo0101() {
    let v1 = vec![1, 2, 3];
    assert_eq!(14, sum_of_square(&v1));

    let mut v2 = vec![1, 2, 3];
    inc_all(&mut v2);
    assert_eq!(vec![2, 3, 4], v2);
    println!("demo01");
}

// 双线程并行计算
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
    a + b
}

pub fn demo0102() {
    assert_eq!(fib(32), 2178309);
}
