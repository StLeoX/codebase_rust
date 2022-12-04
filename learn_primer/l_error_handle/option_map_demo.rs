// Option的map方法，是类型封闭的
// 场景：针对get_shortest(items)，计算shortest的长度，并以Option形式返回
fn get_shortest_length(items: Vec<&str>) -> Option<usize> {
    get_shortest(items).map(|item| item.len())  // map计算仅针对命中Some的匹配值，与unwrap类似
}


////////////////////////////////
// map组合子的大致设计
pub fn map<U, F>(self, f: F) -> Option<U>
    where F: FnOnce(T) -> U {
    match self {
        Some(x) -> Some(f(x)),  // 增加Some嵌套层数
        None -> None,
    }
}


// and_then组合子的大致设计
pub fn and_then<U, F>(self, f: F) -> Option<U>
    where F: FnOnce<T> -> Option<U> {
    match self {
        Some(x) -> f(x),  // 不增加Some嵌套层数
        None -> None,
    }
}
