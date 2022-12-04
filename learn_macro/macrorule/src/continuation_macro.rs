// continuation 执行的剩余部分，也可以称为rest。
// 在Lisp里面，continuation通过head,tail两部分中的tail部分来表示。
// 此处借助continuation的方式完成递归调用。

macro_rules! continuation {
    () => {};
    (trace $name:ident; $($tail:tt)*) => {
        {
            println!(concat!(stringify!($name), " = {:?}"), $name);
            continuation!($($tail)*);
        }
    };
    (trace $name:ident = $init:expr; $($tail:tt)*) => {
        {
            let $name = $init;
            println!(concat!(stringify!($name), " = {:?}"), $name);
            continuation!($($tail)*);
        }
    };
}

#[test]
fn continuation_main() {
    let a = 1;
    let b = "abc";
    let c = (false, 2, 'c');
    continuation!(
        trace a;
        trace b;
        trace c;
        trace b = "Test";
        trace b;
    );
}

// “内部宏”：在宏的内定定义其他的宏，并提供局部可见性
// 采用@来标记内部宏
#[macro_export]
macro_rules! foo {
    (@as_expr $e:expr) => {$e};
    ($($tts:tt)*) => {
        foo!(@as_expr $($tts)*)
    };
}
#[test]
fn main02() {
    assert_eq!(foo!(42), 42);
}
