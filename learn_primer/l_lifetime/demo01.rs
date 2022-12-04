// 使用花括号创建词法作用域。
// 并通过该作用域完成引用语义变量的确定性析构。
fn main() {
    let outer_val = 1;
    let outer_sp = "1".to_string();
    {
        let inner_val = 2;
        outer_val;
        outer_sp;  // String类型变量被自动析构
    }
    println!("{:?}", outer_val);  // correctly, 1
    // println!("{:?}", inner_val);  // cannot find value `inner_val` in this scope
    // println!("{:?}", outer_sp);  // borrow of moved value: `outer_sp`
}