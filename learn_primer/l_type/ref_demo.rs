let x = 5_i32;// typeof x is i32
let x = &5_i32;// typeof x is &i32
let ref x = 5_i32;// typeof x is &i32
let ref x = &5_i32;// typeof x is &&i32


let mut x: &mut i32 = 0;
