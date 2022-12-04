// 封装Union模拟实现内存安全的Tagged Union

// 待模拟的enum
enum EnumDemo {
    I(i32),
    F(f32),
}

////////////////////////////////

// store only
#[repr(C)]
union U {
    i: i32,
    f: f32,
}

// enum Item
#[repr(C)]
struct Item {
    tag: u8,
    value: U,
}

// enum = Items
union MyEnumDemo {
    i: Item,
    f: Item,
}

fn main() {
    let int_0 = MyEnumDemo {
        i: Item {
            tag: b'0',
            value: U { i: 0 },
        },
    };
    let float_0 = MyEnumDemo {
        f: Item {
            tag: b'1',
            value: U { f: 0 },
        },
    };
}
