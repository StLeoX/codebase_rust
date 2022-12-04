// 迭代器适配器使用案例
fn main() {
    let a1 = [1, 2, 3, 4, 5];
    let v1 = a1.iter().map(|x| 2*x).collect::<Vec<i32>>();  // 迭代器的采集
    assert_eq!(&v1[..], [2,4,6,8,10]);

    let a2 = ["1","2","3","a"];
    let v2 = a2.iter().filter_map(|x| x.parse().ok()).collect::<Vec<i32>>();
    assert_eq!(&v2[..],[1,2,3]);

    let a3 = ["a","b","c"];
    for (i,v) in a3.iter().enumerate() {
        println!("i: {}, v:{}", i, v);
    }
}
