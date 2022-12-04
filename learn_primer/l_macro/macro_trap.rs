// 编写macro的陷阱

macro_rules! dead_rule {
    ($e:expr) => { ... };
    ($i:ident +) => { ... };
}

// 这种写法会导致panic，应当先写ident : a concrete Token，再写expr : a abstract Token.

////////////////////////////////
// pattern的定性
// 捕获后确定e为expr类型，再以expr Token的类型传入stringify。
macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}
fn main() {
    // 前者字符串化的是一列标记树
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    // 后者字符串化的则是一个AST expr Token
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));
}

////////////////////////////////
// pattern的定性示例二
macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

fn main() {
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    // 经由capture确定为expr Token，再传入match_token后无匹配
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );
}

////////////////////////////////
/// self ident
macro_rules! what_is {
    (self) => {
        "the keyword `self`"
    };
    ($i:ident) => {
        concat!("the identifier `", stringify!($i), "`")
    };
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i)
    };
}

fn main() {
    println!("{}", what_is!(self)); // output: the keyword `self`
    println!("{}", call_with_ident!(what_is(self))); // output: the keyword `self`
}
