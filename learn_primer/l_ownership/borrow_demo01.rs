// 原始方式：使用返回值归还所有权
// 传递值语义类型：数组类型，是复制语义
fn foo(mut v: [i32; 3]) -> [i32; 3] {
    v[0]=3;
    assert_eq!([3,2,3], v);
    v  // 归还所有权
}

fn main() {
    let v = [1,2,3];  // 严格地，这里应该let mut
    foo(v);  // 函数形实结合的过程也支持模式匹配，或者说immutable在执行Copy Trait的过程中转变为了mutable。
    assert_eq!([1,2,3], v);
}

// 改进方式：使用引用作为函数参数，于是通过借用归还所有权
fn foo(v: &mut [i32; 3]) {
    v[0]=3;
    // assert_eq!(v, [3,2,3]);  // 报错：can't compare `&mut [i32; 3]` with `[{integer}; 3]`
    // 隐式归还所有权
}

fn main() {
    let mut v = [1,2,3];
    foo(&mut v);
    assert_eq!([3,2,3], v);
}

