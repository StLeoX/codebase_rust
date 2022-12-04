use threadpool::ThreadPool;
use std::sync::atomic::*;
use std::sync::Arc;

fn demo01() {
    let pool = ThreadPool::new(4);
    let atom = Arc::new(AtomicUsize::new(0));
    for _ in 0..42 {
        let atom = atom.clone();
        pool.execute(move || {
            atom.fetch_add(1, Ordering::Relaxed);
        });
    }
    pool.join(); // 阻塞主线程并执行
    assert_eq!(42, atom.load(Ordering::Relaxed));
    println!("demo01");
}