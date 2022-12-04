// TDD
fn main() {
    map! {
        1 => "a",
        2 => "b",
        3 => "c",  // 这个称为“结尾逗号”。
    }
}

// 结尾逗号严格添加
macro_rules! map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();  // 使用绝对路径，避免冲突。
            ${
                _map.insert($key, $val);
            }*
            _map
        }  // expr
    };
}

// 改进：结尾逗号自由添加
// 方式一：递归调用消去结尾逗号
macro_rules! map01 {
    ($($key:expr => $val:expr,)*) => { // 注意命中该分支时，逗号的位置
        map01!($($key, $val),*)
    };  // 递归调用
    ($($key:expr => $val:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            ${
                _map.insert($key, $val);
            }*
            _map
        }
    };
}

// 方式二：串行的模式来匹配结尾逗号
// 增加 $(,)* 直接匹配结尾逗号
macro_rules! map02 {
    ($($key:expr => $val:expr),* $(,)*) => {
        {
            let mut _map = ::std::collections::HashMap::new();  // 使用绝对路径，避免冲突。
            ${
                _map.insert($key, $val);
            }*
            _map
        }
    };
}


