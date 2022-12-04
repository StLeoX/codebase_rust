fn hello() {
    println!("hello");
}

fn main() {
    let fn_ptr: fn() = hello;  // 被显式指定为函数指针类型
    println!("{:p}", fn_ptr);
    let fn_item = hello;  // 被隐式推导为函数类型，记为fn() {hello}
    // println!("{:p}", fn_item);  // 不可以指针形式被打印
    hello();
    fn_ptr();  // callable
    (fn_ptr)();
    fn_item();  // callable
    // (fn_item)();
}