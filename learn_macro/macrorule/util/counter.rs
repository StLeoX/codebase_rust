//! 计数器工具宏

// 重复替换方式
macro_rules! replace_expr {
    ($_t:tt, $sub:expr) => {
        $sub
    };
}
macro_rules! count_tts_0 {
    ($($tts:tt),*) => {0usize $(+ replace_expr!($tts, 1usize))*};
}

// 递归方式
macro_rules! count_tts {
    () => {0usize};
    ($_head:tt, $($tail:tt),*) => {1usize+ count_tts!($($tail)*)}; // sep: ','
}

#[test]
fn test_count_tts() {
    assert_eq!(count_tts0!(1, 2, 3), 3);
    assert_eq!(count_tts!(1, 2, 3), 3);
}
