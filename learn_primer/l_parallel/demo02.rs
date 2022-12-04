// scope线程：在scope线程中安全地使用父线程中的引用。
use crossbeam::thread::scope;

pub fn demo01() {
    let a1 = [1, 2, 3];
    scope(|s| {
        // 使用父线程中的引用&a1
        for x in &a1 {  
            s.spawn(move |_| println!("{}", x));
        }
    }).unwrap();
}
