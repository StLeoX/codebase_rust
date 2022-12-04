// Map Wrapper的大致设计
#[derive(Clone)]
pub struct Map<I, F> {
    iter: I,  // 被包装的Iterator
    f: F,  // 待应用的函数
}

impl<B, I: Iterator, F> Iterator for Map<I, F> 
    where F: FnMut(I::Item) -> B
{
    type Item = B;
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)  
        // Iterator Trait内有map存根：
        // fn map<B, F>(self, f: F) -> Map<Self, F>;

    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()  // 委托self.iter进行计算
    }
}