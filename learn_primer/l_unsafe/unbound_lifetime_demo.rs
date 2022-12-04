// 未绑定生命周期

// 从原始指针得到引用，很容易造成“悬垂指针”问题。
fn foo<'a>(input: *const u32) -> &'a u32 {
    unsafe {
        return &*input; // also: &(*input)
    }
}

fn main() {
    let x;
    {
        let y = 1;
        x = foo(&y);
    }
    println!("x: {}", x);
}

// 输出：在Debug模式下编译输出正常结果"x: 1"；
// 但在Release模式下，由于rustc存在Realease模式下的优化。输出异常结果"x: 异常值"。
// 原因：foo函数产生了一个未绑定的生命周期，并跳过了Rust的借用检查。
