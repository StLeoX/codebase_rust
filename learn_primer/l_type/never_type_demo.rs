// never type
#![feature(never_type)]
fn foo()->i32 {
    let x:!={
        return 1 // return expression
    };
    x
}

////////////////////////////////
#![feature(never_type)]
fn bar()->! {
    loop { println!("bar"); }  // 返回loop表达式是never类型的
}


fn main() {
    let a = if false{
        bar();  // call, and get a never_type
    } else {
        1
    };
    assert_eq!(a, 1);
    println!("ok");
}

