// Ant type usecase: downcast_ref

use std::any::Any;
#[derive(Debug)]
enum E {H, He, Li}

#[derive(Debug)]
struct S {
    x: u8,
    y: u8,
    z: u16,
}

fn print_any(a: &dyn Any) {  // dyn trait
    if let Some(x) = a.downcast_ref::<u32>() {
        println!("u32 {:x}", x);
    } else if let Some(x) = a.downcast_ref::<E>() {
        println!("E {:?}", x);
    } else if let Some(x) = a.downcast_ref::<S>() {
        println!("S {:?}", x);
    } else {
        println!("defualt");
    }

}

fn main() {
    print_any(& 0x1234_u32);
    print_any(& E::He);
    print_any(& S{x:1, y:2, z:3});
    print_any(& "any ohter");
}