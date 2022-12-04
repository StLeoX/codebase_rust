// if-not semantic
macro_rules! unless {
    ($cond:expr, $body:expr) => {
        if !$cond {
            $body
        };
    };
}

fn main() {
    let (a, b) = (1, 2);
    unless!(a > b, println!("{}", b - a));
}


////////////////////////////////
// macro parser 概要设计
fn macro_parser(
    sess: ParseSession,  // context
    tts: TokenStream,  // input
    ms: &[TokenTree]  // output
) -> NamedParseResult;  // return status

