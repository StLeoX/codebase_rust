// Result的大致设计
#[must_use]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}


use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("foo.txt")?;  // 注意问号这个语法糖，会在发生错误时返回Error对象
    Ok(())
}