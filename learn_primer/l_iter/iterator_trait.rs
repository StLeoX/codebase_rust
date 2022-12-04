// Iterator Trait的大致设计
pub trait Iterator {
    type Item;  // 需要指定元素类型
    fn next(&mut self) -> Option<Self::Item>;  // 需要实现next方法，迭代为后继元素。（Option因为尾元素的后继为None）
    fn size_hint(&self) -> (usize, Option<usize>);  // 需要实现size_hint方法，描述迭代器剩余长度的边界信息。（tuple）
}


////
// IntoIterator Trait的大致设计
pub trait IntoIterator {
    type Item;
    type IntoIter: IntoIterator<Item = Self::Item>;  
    fn into_iter(self) -> Self::IntoIter;  // IntoIterator构造的是IntoIter
}

// IntoIter结构
struct IntoIter<T> {
    buf,
    cap,
    ptr,
    end,
}

// IntoIter IS-A Iterator
impl Iterator for IntoIter;

// 为Vec混入into_iter方法
impl<E> IntoIterator for Vec<E> {
    type Item = E;
    type IntoIter = IntoIter<E>;
    fn into_iter(mut self) -> IntoIter<E> {
        // ...
    }
};