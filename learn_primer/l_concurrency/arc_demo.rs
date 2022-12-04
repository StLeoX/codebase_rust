use std::thread;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;


// Arc<T>
fn demo01() {
    // Arc<T>是不可变的，需要借助内部可变性类型组合为新类型。
    let s = Arc::new("loading".to_string());
    for _ in 0..3 {
        let s_clone = s.clone();
        thread::spawn(move || {
            s_clone.push_str(".");
        });
    }
}

// Arc<RefCell<T>>
fn demo02() {
    // 组合RefCell后，因为RefCell缺少Sync的实现，故不兼容。
    let s = Arc::new(RefCell::new("loading".to_string()));
    for _ in 0..3 {
        let s_clone = s.clone();
        thread::spawn(move || {
            let s_clone = s_clone.borrow_mut();
            s_clone.push_str(".");
        });
    }
}

// Arc<Mutex<T>>
fn dmeo03() {
    let s = Arc::new(Mutex::new("loading".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(".");
        });
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }
}