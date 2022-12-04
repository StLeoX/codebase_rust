// 简单宏的编写
macro_rules! say_hello {
    ($name: expr) => {println!("Hello {}", $name);};
}

fn demo01() {
    say_hello!("Macro");
}
////////////////////////////////
// 重复模式的提取与利用
macro_rules! say_hello {
    ($($name: expr), *) => {$( println!("Hello {}", $name); )*};
}

fn demo02() {
    say_hello!("Macro","Leo");
}

// stdout:
// Hello Macro
// Hello Leo

////////////////////////////////
// map! Macro Implement, Not Builtin
use std::collections::HashMap;

macro_rules! map {
     // 比较有意思的是，分号在此处无法scan，只能使用：note: allowed there are: `=>`, `,` or `;`，所以使用=>。
    ($($key: expr => $value: expr), *) => {{ 
        let mut hmap = HashMap::new();
        $(hmap.insert($key, $value); )*
    }};
}


fn demo03() {
    let map1 = map!{
        "k1" => "v1",
        "k2" => "v2"  // 注意：末尾条目不允许出现逗号，否则无法提取重复的模式。
    };
    println!("{:?}", map1);
}