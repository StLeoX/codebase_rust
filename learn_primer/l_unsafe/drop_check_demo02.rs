// 使用#[may_dangle] 属性，指示编译器此处可能产生悬垂指针，并在编码时避免悬垂指针。

struct WrappedBox<T> {
    value: Box<T>,
}

#![feature(allocator_api, dropck_evepatch)]

// 不访问类型T存储的数据
unsafe impl<#[may_dangle] T> Drop for WrappedBox<T> {
    fn drop(&mut self) {
        println!("Dropping");
        let p = self.value as *mut _;
        System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
    }
}


// 访问类型T存储的数据，必然导致悬垂指针。于是写成了不安全的程序。
// 不访问类型T存储的数据
unsafe impl<#[may_dangle] T> Drop for WrappedBox<T> {
    fn drop(&mut self) {
        // read self
        println!("Dropping {}", self.value);
        let p = self.value as *mut _;
        // read self
        // ptr::read(p);
        System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
    }
}


