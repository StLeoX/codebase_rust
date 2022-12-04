fn main() {
    let a = [1,2,3];
    let r1 = a.iter().any(|&x| x != 2);
    let r2 = a.iter().any(|x| *x != 2);
    assert_eq!(r1, true);
    assert_eq!(r2, true);

}


fn main() {
    let a = vec![1,2,3];
    let sum1 = a.iter().fold(0, |acc,x| acc + x);
    let sum2 = a.iter().fold(0, |acc,x| acc + *x);
    let sum3 = a.iter().fold(0, |acc,&x| acc + x);
    // 针对Vec<T>，是否借用都是可以的
    assert_eq!(sum1, 6);
    assert_eq!(sum2, 6);
    assert_eq!(sum3, 6);
    
    ////
    let sum4 = a.into_iter().fold(0, |acc,x| acc + x);
    // let sum4 = a.into_iter().fold(0, |acc,&x| acc + x);
    assert_eq!(sum4, 6);

}