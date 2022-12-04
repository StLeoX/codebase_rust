// if-let 表达式使用
fn demo01() {
    let v1 = true;
    let mut v2 = 0;
    if let true = v1 {
        v2 = 1;
    }
    assert_eq!(v2, 1);
}


// while-let 表达式使用
// 原始：loop-match写法
fn demo02() {
    let mut v=vec![1,2,3,4,5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}


// 简化：while-let写法
fn demo03() {
    let mut v=vec![1,2,3,4,5];
    while let Some(x) = v.pop() {
        println!("{}",x);
    }
}

