// 什么时候需要用到生命周期参数
// 输入借用，输出借用
fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 输入借用，输出副本
fn the_longest_copy<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1.clone() } else { s2.clone() }
}

fn main() {
    let a = "hello".to_owned();
    let a_r = &a;
    {
        let b = "hi".to_owned();
        // let result = the_longest(a_r, &b);
        let result = the_longest_copy(a_r, &b);
        assert_eq!(result, "hello");
    }
}


////
// 指定生命周期参数之间的关系
// 'b包含'a，即'a是'b的子集，较短。
fn the_longest<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}