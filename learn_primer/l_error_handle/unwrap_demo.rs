// 重写Some-macth例子，使用unwrap系列函数
fn show_shortest(items: Vec<&str>) -> &str {
    get_shortest(items).unwrap();

    get_shortest(items).unwrap_or("no_found");

    get_shortest(items).unwrap_or_else(|| "no_found");

    get_shortest(items).expect("no_found");

}
