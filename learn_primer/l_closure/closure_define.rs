// 闭包写法
fn demo01() {
    let out=1;
    fn add(a:i32, b:i32) -> i32 {a+b}// 函数声明不支持闭包
    let closure01=|a:i32, b:i32| -> i32 {a+b+out};// 闭包写法1
    let closure02=|a,b| a+b+out;// 闭包写法2
}

// 将闭包作为参数的高阶函数
fn math<F: Fn() -> i32>(op:F) -> i32 {
    op()
}

fn demo02() {
    let a=2;
    let b=3;
    assert_eq!(math(|| a+b), 5); // || a+b是一个闭包，空参数，a+b是返回值
    assert_eq!(math(|| a*b), 6);
}


// 将闭包作为返回值的高阶函数
fn multi2_factory() -> impl Fn(i32) -> i32 { // (impl Fn(i32) -> i32) 整体是闭包的型
    let base = 2;
    move |x| x*base 
    // 注意move关键字的使用：防止闭包中的自由变量base在调用闭包之前被释放，形成悬垂。
    // 报错为：may outlive borrowed value `base`
}

fn demo03() {
    let m2 = multi2_factory();
    assert_eq!(4, m2(2));
}