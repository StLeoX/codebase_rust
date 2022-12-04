// ref: https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/spilt.2.pim-README.md

#[macro_use(recurrence)]
extern crate recurrence;

use crate::recurrence::RecurrenceIterator;

#[test]
fn test_iter() {
    let fib = RecurrenceIterator::new([1, 1]);
    let fibs: Vec<u64> = fib.take(3).collect();
    assert_eq!(vec![1, 1, 2], fibs);
}

#[test]
fn test_macro() {
    let fib = recurrence!(r[n] = 0,1, ... ,r[n-1]+r[n-2]);
    for e in fib.take(10) {
        println!("{}", e)
    }
}
