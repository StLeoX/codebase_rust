// Newtype Define
struct PrintableString(&'static str);

impl Drop for PrintableString {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}