fn join(s: &String) -> String {
    let gain_ownership = *s;
    "a".to_string + &gain_ownership  // consume the dereference of s
}

fn main() {
    let x = "b".to_string();
    join(&x);
}