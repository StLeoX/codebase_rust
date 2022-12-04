// Future Trait 概要设计
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut self>, lw: &LocalWaker) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

