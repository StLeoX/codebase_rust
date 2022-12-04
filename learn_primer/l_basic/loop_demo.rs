// 嵌套循环和标签

#![allow(unreachable_code)]
fn demo01() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 这只是中断内部的循环
            //break;
            // 这会中断外层循环
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

// 从 loop 循环中返回
fn demo02() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
