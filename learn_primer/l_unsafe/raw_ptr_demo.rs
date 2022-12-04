// 引用方式 创建原生指针
// 该方式创建的指针指向数据的整块堆内存
fn demo01() {
    let mut s = "abc".to_owned();
    let s_r1 = &s as *const String;
    let s_r2 = &mut s as *mut String;
    assert_eq!(s_r1, s_r2);
    unsafe {
        println!("{}", *s_r1); // abc
        println!("{}", *s_r2); // abc
    }
}

// 创建空指针
fn demo02() {
    let p1: *const u8 = std::ptr::null();
    assert!(p1.is_null());
}

// 获取方式 创建原生指针
fn demo03() {
    let s = "abc";
    let s_r: *const u8 = s.as_ptr();
    assert!(!s_r.is_null());
    unsafe {
        println!("{}", *s_r as char); // a
    }

    let mut v = [1, 2, 3]; // 本质是在操作切片，所以array/vector类似
    let v_r: *mut u32 = v.as_mut_ptr();
    assert!(!v_r.is_null());
    unsafe {
        println!("{}", *v_r); // 1
    }
}

// as_ptr + offset
fn demo04() {
    let s = "abc";
    let s_r: *const u8 = s.as_ptr();
    unsafe {
        // 解引用首地址
        println!("{:?}", *s_r as char); // 'a'
                                        // 使用as char指明转出类型，否则打印的是Unicode码值
        println!("{:?}", *s_r.offset(1) as char); // 'b'
                                                  // offset不进行越界检查
        println!("{:?}", *s_r.offset(4) as char); // 's'
    }
}

// read/write
fn demo05() {
    // as_ptr 指向起始地址
    let s = "abc".to_owned();
    let s_r: *const u8 = s.as_ptr();
    unsafe {
        assert_eq!(s_r.read() as char, 'a');
    }

    // [..].as_ptr 指向整体
    let a = [0i32, 1, 2, 3];
    let a_r = a[..].as_ptr() as *const [i32; 4];
    unsafe { assert_eq!(a_r.read(), [0i32, 1, 2, 3]) }

    // & 指向整体
    let v = vec![0i32, 1, 2, 3];
    let v_r = &v as *const Vec<i32>;
    unsafe {
        assert_eq!(v_r.read(), vec![0i32, 1, 2, 3]);
    }

    // &mut 指向整体
    let mut x = "";
    let x_r = &mut x as *mut &str;
    unsafe {
        x_r.write("abc");
        assert_eq!(x_r.read(), "abc");
    }
}

// Warning: free(): double free detected in tcache 2

// replace/swap
fn demo06() {
    // replace 语义一
    let mut v = vec![1i32, 2];
    let v_r: *mut i32 = v.as_mut_ptr();

    unsafe {
        // as_ptr 指向首地址，所以replace仅替换首个元素
        let old_v = v_r.replace(5);
        assert_eq!(1, old_v);
        assert_eq!([5, 2], &v[..]);
    }

    // replace 语义二
    let mut v2 = vec![1i32, 2];
    let v2_r = &mut v2 as *mut Vec<i32>;
    unsafe {
        // &mut 指向整体，所以replace替换整体
        let old_v2 = v2_r.replace(vec![3, 4, 5]);
        assert_eq!([1, 2], &old_v2[..]);
        assert_eq!([3, 4, 5], &v2[..]);
    }

    // replace/swap 引起的内存操作歧义
    let mut a = [0i32, 1, 2, 3];
    // as 转出的数组类型，具备截取语义
    let x = a[0..].as_mut_ptr() as *mut [u32; 2];
    let y = a[1..].as_mut_ptr() as *mut [u32; 2];
    unsafe {
        assert_eq!([0, 1], x.read());
        assert_eq!([1, 2], y.read());
        x.swap(y);
        assert_eq!([1, 0, 1, 3], a);
    }
}

// 在std::mem模块中提供了一个内存安全的swap方法，其函数签名为：
// fn swap<T>(x:&mut T，y:&mut T)，注意其参数为&mut 可变引用。
