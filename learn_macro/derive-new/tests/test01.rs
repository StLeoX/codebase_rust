// 功能测试
use derive_new::New;

// 具名结构体
#[derive(Debug, PartialEq, New)]
pub struct NameS1 {}

#[derive(Debug, PartialEq, New)]
pub struct NameS2 {
    pub i: i32,
    pub s: String,
}

// 单元结构体
#[derive(Debug, PartialEq, New)]
pub struct UnitS;

// 元组结构体
#[derive(Debug, PartialEq, New)]
pub struct TupleS(i32, i32);

//// testcases
#[test]
fn test_empty_struct() {
    let a = NameS1::new();
    assert_eq!(a, NameS1 {});
}

#[test]
fn test_normal_struct() {
    let a = NameS2::new(1, "a".to_string());
    assert_eq!(a, NameS2 { i: 1, s: "a" });
}

#[test]
fn test_unit_struct() {
    a = UnitS::new();
    assert_eq!(a, UnitS);
}

#[test]
fn test_tuple_struct() {
    let a = TupleS::new(1, 2);
    assert_eq!(a, TupleS(1, 2));
}
