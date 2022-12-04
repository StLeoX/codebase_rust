// 跨语言的堆内存管理
// 原则：如果堆内存是Rust中分配的，应该交由Rust来释放。也就是说，在Rust中要以unsafe形式编写内存释放的逻辑。

////////////////////////////////
/// A KV DataBase example

use libc::{c_char, c_uint};  // 通过libc crate导入可用的C类型
use std::ffi:CStr;

use std::collections::HashMap;

pub struct KVDataBase {
    db: HashMap<String, u32>,
}

impl KVDataBase {
    fn new() -> KVDataBase {
        KVDataBase { db: HashMap::new() }
    }

    fn insert(*mut self) {
        for i in 1..100{
            self.db.insert(format!("{:02}", i), i);
        }
    } 

    fn get(*self, key:&str) ->u32{
        self.db.get(key).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern fn db_new() -> *mut KVDataBase {
    Box::into_raw(Box::new(KVDataBase::new()))
}

#[no_mangle]
pub extern fn db_insert(ptr: *mut KVDataBase) {
    let db_ptr = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    db_ptr.insert();
}

#[no_mangle]
pub extern fn db_query(ptr: *const KVDataBase, key: *const c_char) -> c_uint {
    let db_ptr = unsafe {
        assert!(!ptr.is_null());
        & *ptr
    };

    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key)
    }

    let query_key = key.to_str().unwrap();
    db_ptr.get(query_key);
}

// 既然存在db_new，就应当存在db_delete，在Rust代码中。
#[no_mangle]
pub extern fn db_delete(ptr:*mut KVDataBase) {
    if ptr.is_null() {return}
    unsafe {Box::from_raw(ptr);}
}


