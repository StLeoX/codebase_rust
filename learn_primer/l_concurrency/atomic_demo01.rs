use std::sync::Arc;
use std::sync::atomic::{AtomicUsize. Ordering};
use std::thread;

// 使用原子类型实现一个自旋锁（spinlock）的原型。
// 注意到spinlock仍然是双态锁。
fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    })
    while spinlock.load(Ordering::SeqCst) != 0 { } // 自旋操作
    if let Err(e) = thread.join() {
        println!("Err: {}", e);
    }
}


// std::sync::atomic::Ordering 定义的五种内存顺序
enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}