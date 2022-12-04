// 为结构实现TryFrom Trait
// TryFrom和TryInto在实现上是关联的，通常实现TryFrom，就完成了对TryInto的实现。
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(u32);

impl TryFrom<u32> for EvenNumber {
    type Error = ();  // 类型实例化Result::Error

    fn try_from(input: u32) -> Result<Self, ()> {
        if input % 2 == 0 {
            Ok(EvenNumber(input))
        } else {
            Err(())
        }
    }
}

fn main() {
    // testing TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(7), Err(()));

    // testing TryInto
    let result: Result<EvenNumber, ()> = 8u32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result2: Result<EvenNumber, ()> = 7u32.try_into();
    assert_eq!(result2, Err(()));

}