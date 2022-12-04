// Option的最小实现：描述清楚Some与None的选择关系即可
enum Option {
    Some(i32),
    None,
}

// 实例：如何使用Option
fn get_shortest(items: Vec<&str>) -> Option<&str> {
    if items.len() > 0 {
        let mut shortest = items[0];
        for item in items.iter() {
            if item.len() < shortest.len() { shortest = item; }
        }
        Some(shortest)
    } else {
        None  // empty
    }
}

fn show_shortest(items: Vec<&str>) -> &str {
    match get_shortest(items) {
        Some(shortest) => shortest,
        None => "no_found",  // slug format
    }
}