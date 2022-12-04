// 生命周期的型变关系

// 以下是不安全的代码
struct MyCell<T> {
    value: T,
}

impl<T: Copy> MyCell<T> {
    fn new(value: T) -> MyCell<T> {
        MyCell { value: value }
    }

    fn get(&self) -> T {
        self.value
    }

    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            // Rust 不允许直接将不可变借用转为可变原生指针；
            // 所以需要先转为不可变原生指针。
            ptr::write(&self.value as *const _ as *mut _, value);
        }
    }
}

fn step1<'a>(cell_r: &MyCell<&'a i32>) {
    let a = 1;
    step2(&a, cell_r);
    println!("step1: {}", cell_r.value);
}

fn step2<'b>(val_r: &'b i32, cell_r: &MyCell<&'b i32>) {
    cell_r.set(val_r);
}

fn main() {
    let cell1 = MyCell::new(&10);
    step1(&cell1);
}

// 内存不安全问题：被调用的step2执行完，&val_r引用遭释放，则step1中的&a成为悬垂指针。

// 为什么borrow checker未报错？
// “忘记原始生命周期”

////////////////////////////////
/// 使用PhantomData<T> 修正MyCell结构

struct MyCell<T> {
    value: T,
    mark: PhantomData<fn(T)>, // 增加该字段，以指明MyCell结构为逆变类型
}
