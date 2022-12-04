// 自定义一般属性的功能测试

use l_procmacro::attr_with_args;

#[attr_with_args("foo")]
fn foo() {}

#[test]
fn test_foo() {
    assert_eq!(foo(), "foo");
}

