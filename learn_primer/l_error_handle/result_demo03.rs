// try! 的大致设计
macro_rules! try {
    ($expr: expr) => (match $expr {
        $crate::result::Result::Ok(value) => value,
        $crate::result::Result::Err(err) => {
            return $crate::result::Result::Err(
                $crate::convert::From::from(err)
            )
        }
    })
}