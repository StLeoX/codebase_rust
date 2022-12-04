use std::thread;

fn main() {
    let mut v = Vec::new();
    for i in 0..5 {
        // spawn接收Func/Closure类型的参数，作为thread执行的入口。
        let child = thread::spawn(move || {
            println!("child id: {:?}", i);
        });
        v.push(child);
    }

    println!("join before");
    for child in v {
        child.join();
    }
    println!("join after");
}

// spawn大致设计
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static {
    Builder::new().spawn().unwrap();
}