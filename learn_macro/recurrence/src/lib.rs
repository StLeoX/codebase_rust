// 实现思路：迭代首先需要impl Iterator；
// 调用take方法需要impl Index。

use std::ops::Index;

pub struct RecurrenceIterator {
    nums: [u64; 2], // 滑动窗口
    pos: usize,     // 迭代次数
}

struct IndexOffset<'a> {
    slice: &'a [u64; 2], //
    offset: usize,
}

impl<'a> Index<usize> for IndexOffset<'a> {
    type Output = u64;
    fn index<'b>(&'b self, index: usize) -> &'b u64 {
        use std::num::Wrapping;
        let index = Wrapping(index);
        let offset = Wrapping(self.offset);
        let window = Wrapping(2);
        let real_index = index - offset + window;
        &self.slice[real_index.0]
    }
}

impl Iterator for RecurrenceIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 2 {
            let next = self.nums[self.pos];
            self.pos += 1;
            Some(next)
        } else {
            let next = {
                let a = IndexOffset {
                    slice: &self.nums,
                    offset: self.pos,
                };
                a[self.pos - 1] + a[self.pos - 2]
            };
            self.nums[0] = self.nums[1];
            self.nums[1] = next;
            self.pos += 1;
            Some(next)
        }
    }
}

impl RecurrenceIterator {
    pub fn new(inits: [u64; 2]) -> RecurrenceIterator {
        RecurrenceIterator {
            nums: inits,
            pos: 0,
        }
    }
}

/// macro logic
#[macro_export]
macro_rules! recurrence {
    (r[n]: $sty:ty = $($inits:expr),+ , ... ,$recur:expr) => {
        todo!("recurrence")
    }
}
