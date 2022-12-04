// 函数指针枚举体
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn demo01() {
    let x: fn(u8,u8,u8,u8) -> IpAddr = IpAddr::V4;
    let y: fn(String) -> IpAddr = IpAddr::V6;
    let home: IpAddr = IpAddr::V4(127,0,0,1);
}

