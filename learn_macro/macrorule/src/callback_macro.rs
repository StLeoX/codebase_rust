//! 在宏中完成回调

macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}
macro_rules! expand_to_larch {
    () => {
        larch
    };
}
macro_rules! recognise_tree {
    (larch) => {
        println!("#1, 落叶松。")
    };
    (redwood) => {
        println!("#2, THE巨红杉。")
    };
    (fir) => {
        println!("#3, 冷杉。")
    };
    (chestnut) => {
        println!("#4, 七叶树。")
    };
    (pine) => {
        println!("#5, 欧洲赤松。")
    };
    ($($other:tt)*) => {
        println!("不懂，可能是种桦树？")
    };
}

#[test]
fn callback_main() {
    recognise_tree!(expand_to_larch!());
    call_with_larch!(recognise_tree);
}
