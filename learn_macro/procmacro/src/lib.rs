extern crate proc_macro;

use self::proc_macro::TokenStream;

// 派生属性宏的实现
#[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    assert!(input.contains("struct A;"));
    r#"
        impl A {
            fn a(&self) -> String {
                format!("test")
            }
        }
    "#
    .parse()
    .unwrap() // parse raw string, which implements a for A.
}

// 一般属性宏的实现
// #![feature(custom_attribute)]
#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let input = input.to_string();
    // hard-coding implements echo-args.
    format!("fn foo() -> &' static str {{ {:?} }}", args)
        .parse()
        .unwrap()
}

// Bang宏形式重写map macro
#![feature(proc_macro_hygiene)]
#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    let input = input.to_string().trim_right_matches(',').split(",");

    let insert_cmds: Vec<String>= input.map(|x| {
        let mut data = if x.contains(":") {x.split(":")}
                        else {x.split("=>")};  // 兼容两种连接符
        let (key, val) = (data.next().unwrap(), data.next().unwrap());
        format!("_map.insert({}, {})",key, val)  // 与声明宏很类似，但是是普通的非宏语法写的
    }).collect();
    
    let count:usize= insert_cmds.len();
    let cmd_str = insert_cmds.iter().map(|cmd| format!("{};", cmd)).collect::<String>();

    // 渲染Rust source code
    format!("{{
        let mut _map = ::std::collections::HashMap::with_capacity({});
        {}
        _map
    }}", count, cmd_str).parse().unwrap()
}

// 实际上，除了通过普通文本来渲染Rust code外（无可避免的hard-coding），还有：
// porc_macro包提供了Token相关的结构，以及将Token转为TokenStream的quote! macro，但是目前比较难以使用。

