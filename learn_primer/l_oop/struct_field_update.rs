struct book {
    title: &'a str,
    version: i32,
    author: String
}

// 更新语法：针对成员全为复制语义
fn demo01() {
    let book1 = Book { title: "Book 1", version: 1, author: "John"};
    let book2 = Book { title: "Book 2", ..book1};  // 复用了book1的成员，并更新title成员
    println!("{:}", book1);
    println!("{:}", book2);

}


// 更新语法：针对成员含有移动语义
fn demo02() {
    let book1 = Book { title: "Book 1", version: 1, author: String::from("John)"};
    let book2 = Book { title: "Book 2", ..book1};  // 复用了book1的成员，并更新title成员
    // book1.author 移动到book2中，book1整个结构失效
    // println!("{:}", book1);
    println!("{:}", book2);

}

