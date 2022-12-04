// 自定义Bang宏的功能测试

#![feature(proc_macro_hygiene)]
use l_procmacro::map;

#[test]
fn test_map() {
    let hm1 = map! {"a": 1, "b": 2, "c": 3};
    assert_eq!(hm1.get("a"), 1);

    let hm2 = map! {"a"=>1, "b"=>2, "c"=>3};
    assert_eq!(hm2.get("a"), 1);
}


