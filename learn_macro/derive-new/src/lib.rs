extern crate proc_macro;

use {
    self::proc_macro::TokenStream,
    proc_macro2,
    quote::*,
    syn::{parse_macro_input, DeriveInput, Token},
};

// 定义派生属性New
#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput); // 该宏需要指明目标类型
    let result = match ast.data {
        syn::Data::Struct(ref s) => new_for_struct(&ast, &s.fields), // 命中待处理的类型：Struct
        _ => panic!("Unexpected"),
    };
    result.into()
}

/**
 * DeriveInput结构体包含了五个字段，它们记录的信息如下：
· attrs，实际为 Vec＜syn::Attribute＞类型，syn::Atrribute 代表属性，比如＃[repr（C）]，使用Vec＜T＞代表可以定义多个属性。用于存储作用于结构体或枚举体的属性。
· vis，为syn::Visibility类型，代表结构体或枚举体的可见性。
· ident，为syn::Ident类型，将会存储结构体或枚举体的名称。
· generics，为syn::Generics，用于存储泛型信息。
· data，为syn::Data，包括结构体、枚举体和联合体这三种类型。

* DeriveInput结构体还实现了一个重要的trait: Parse Trait。
*/

// 结构初始化函数，体现出一种很强的工厂函数的味道
// 设计：因为之前match命中Struct，所以不再使用Result包装
fn new_for_struct(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => new_impl(&ast, Some(&fields.named), true),
        syn::Fields::Unit => new_impl(&ast, None, false),
        syn::Fields::Unnamed(ref fields) => new_impl(&ast, Some(&fields.unnamed), false),
    }
}

// new函数的具体实现
fn new_impl(
    ast: &syn::DeriveInput,
    fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
    named: bool,
) -> proc_macro2::TokenStream {
    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (mut new, doc) = (
        syn::Ident::new("new", proc_macro2::Span::call_site()),
        format!("Constructing a new {}.", struct_name),
    );

    let args = "todo";
    let inits = "todo";

    // 模板渲染，通过在环境中捕获变量，来渲染以下snippet，非字符串形式。
    quote! {
        // todo: method cannot be called on `&str`.
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new(#(#args),*) -> Self {
                #struct_name #inits
            }
        }
    }
}
