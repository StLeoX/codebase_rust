//! visit & evaluate，直接遍历求值

use crate::frontend::{NodeType, Tree};

pub trait Visitable<T> {
    fn visit(&self) -> T;
}

impl Visitable<f64> for Tree {
    fn visit(&self) -> f64 {
        let result = 0f64;
        //
        while !(self.left.is_none() && self.right.is_none()) {
            match self.ty {
                NodeType::Number => result = self.value,
                NodeType::Plus => result = self.left.unwrap().visit() + self.right.unwrap().visit(),
                NodeType::Minus => {
                    result = self.left.unwrap().visit() - self.right.unwrap().visit()
                }
                NodeType::Mul => result = self.left.unwrap().visit() * self.right.unwrap().visit(),
                NodeType::Div => result = self.left.unwrap().visit() / self.right.unwrap().visit(),
                _ => (),
            }
        }
        result
    }
}
