// 自定义派生属性的功能测试
#[macro_use]
extern crate l_procmacro;

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
    assert_eq!("test".to_string(), A.a());
}
