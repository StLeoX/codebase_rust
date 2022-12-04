#[derive(Debug, PartialEq)]
struct A(i32);  //New Type

#[derive(Debug, PartialEq)]
struct B(i32, i32);  // Tuple Structure

trait AB {
    fn new(i: i32) -> Self;
}

impl AB for A {
    fn new(i: i32) -> A { A(i) }
}

impl AB for B {
    fn new(i: i32) -> B { B(i, i + 10) }  // Tuple Structure Init
}

fn create_ab<T: AB>(i: i32) -> T {
    T::new(i)
}

fn main() {
    let a: A = create_ab(10);
    assert_eq!(a, A::new(10));
    let b = create_ab::<B>(20);
    assert_eq!(b, B::new(20));
}