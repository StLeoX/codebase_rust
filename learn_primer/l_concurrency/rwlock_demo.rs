use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(3);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 3);
        assert_eq!(*r2, 3);
    }
    // 串行花括号，便于销毁r1，r2。
    {
        let mut w1 = lock.write().unwrap();
        *w1 += 1;
        assert_eq!(*w1, 4);
    }
    
}