// 封装unsafe rust code，使之满足内存安全约束

// Vec<T> insert 概要设计
pub fn insert(&mut self, index: usize, value: T) {
    let len = self.len();
    assert!(index >= 0 && index <= len); // 越界检查
    if len == self.buf.cap() {
        self.reserve(1); // 满则扩容
    }
    // 做到上述两点，就满足了*内存安全约束*，以下是待封装的unsafe code
    unsafe {
        let p = self.as_mut_ptr().offset(index as isize);
        ptr::copy(p, p.offset(1), len - index); // 插入时的顺序移动
        ptr::write(p, value); // 插入元素
        self.set_len(len + 1);
    }
}

// Error: error: `self` parameter is only allowed in associated functions
// 所以insert实现在impl Vec里面。
