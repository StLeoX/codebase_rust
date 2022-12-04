// Rust提供了一个函数std::mem::forget来阻止drop的自动调用。
struct A;
struct B;

struct Foo {
    a: A,
    b: B,
}

impl Foo {
    fn foo(self) -> (A, B) {
        (self.a, self.b)
    }
}

// 如果尝试实现Drop，会报错：
// cannot move out of type `Foo`, which implements the `Drop` trait
impl Drop for Foo {
    fn drop(&mut self) {
        // do something.
    }
}

////////////////////////////////
/// 避免上述报错，以便重写Drop
/// 原理在于：通过mem “伪装” a,b的初始化，从而避免初始化检查报错。
use std::mem;
impl Foo {
    fn foo(mut self) -> (A, B) {
        let a = mem::replace(&mut self.a, unsafe { mem::uninitialized() });
        let b = mem::replace(&mut self.b, unsafe { mem::uninitialized() });
        mem::forget(self); //forget(self.a), forget(self.b)
        (a, b)
    }
}

////////////////////////////////
/// 实际上，forget<T>就是基于ManuallyDrop实现：
use std::mem::ManuallyDrop;
pub fn forget_impl<T>(t: T) {
    ManuallyDrop::new(t);
}
// 通过创建一个需要手动析构的ManuallyDrop实例，来避免自动析构。

// ManuallyDrop的概要设计
#[allow(unions_with_drop_fields)]
#[derive(Copy)]
pub union ManuallyDrop<T> {
    value: T,
} // 利用了“Rust不会为union自动实现Drop”的性质。
impl<T> ManuallyDrop<T> {
    pub const fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value: value }
    }
    pub unsafe fn drop(slot: &mut ManuallyDrop<T>) {
        ptr::drop_in_place(&mut slot.value) // unsafe call；与普通drop相似的委托调用。
    }
}
