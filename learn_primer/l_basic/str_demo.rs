fn main() {
    let truth: &str = "Rust大法好。";
    let truth_ptr=truth.as_ptr();
    
    let trust: &'static str = "The Rust";
    let trust_ptr=trust.as_ptr();
    
    let s = unsafe {
        let slice1 = std::slice::from_raw_parts(trust_ptr,trust.len());
        std::str::from_utf8(slice1)
    };
    
    assert_eq!(s, Ok(trust));
}