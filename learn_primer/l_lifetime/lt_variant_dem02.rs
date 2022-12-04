// fn(T) -- the only Contravariant

trait A {
    fn foo(&self, s: &'static str);
}

struct B;

impl A for B {
    fn foo(&self, s: &'static str) {
        println!("{}", s);
    }
}

impl B {
    fn bar(&self, s: &'static str) {
        println!("{}", s);
    }
}

fn main() {
    // Okkk!
    B.foo("foo");
    // Errr!
    // let s = "bar".to_owned();
    // B.bar(&s);  // borrowed value does not live long enough
}
