// 声明宏的展开类似于在“半沙盒”中进行，具备部分卫生性
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}
let four = using_a!(a / 10);  // error: unresolved name `a`


// “泄漏”宏的沙盒中的变量，通过ident Token回传。
macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let $a = 42;
            $e
        }
    }
}

fn main() {
    let four = using_a!(a, a / 10);
    println!("{}", four);
}