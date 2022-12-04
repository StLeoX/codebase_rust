// trait对象的生命周期
trait Tr {}
struct St<'a> {
    x: &'a i32,
}

impl<'a> Tr for St<'a> {
    // todo
}

fn main() {
    let a=1;
    let st1 = Box::new(St { x: &a });
    let tr1 = st1 as Box<dyn Tr>;  // as转入类型即是trait对象。
}
