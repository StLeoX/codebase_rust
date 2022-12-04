struct book {
    title: &'a str,
    version: i32,
    author: String
}

// 使用Default Trait实现结构成员的缺省语义
// 仅针对引用语义成员，在这里包括author
impl Default for Book {
    fn default() -> Self {
        Book {
            title = "",
            version: 0,
            author: String::default(),
        }
    }
}