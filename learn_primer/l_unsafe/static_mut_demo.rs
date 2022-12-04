// Unsafe Demo in String Lib
unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
    String { vec: bytes }
}

// 修改可变静态变量 是unsafe操作
static mut COUNTER: usize = 0;

fn main() {
    let inc = 1;
    unsafe {
        COUNTER += inc;
        assert_eq!(COUNTER, 1);
    }
}
