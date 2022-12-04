//! AST细化与强制转换
// 在替换tt时，Rust的解析器并不十分可靠。
// 所以在确定AST类型后应当对tt进一步细化。

#![allow(dead_code)]
macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}
macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}

fn _main() {
    as_item! ({struct Dummy;})
    as_stmt! (let as_pat!(_) = as_expr(1);)
}
