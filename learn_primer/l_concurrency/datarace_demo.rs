// 数据竞争示例

use std::thread;

static mut g_v:i32 = 0;

fn unsafe_inc() -> i32 {
    unsafe {
        g_v += 1;
        g_v
    }
}

fn main() {
    // 子线程异步LMS
    let child = thread::spawn(move || unsafe {
        for _ in 0..10 {
            unsafe_inc();
            unsafe { println!("child {}", g_v); }
        }
    });

    // 主线程LMS
    for _ in 0..10 {
        unsafe_inc();
        unsafe { println!("main {}", g_v); }
    }
    child.join();

}