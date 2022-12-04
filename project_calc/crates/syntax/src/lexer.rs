//! lexer
use logos::Logos;

// token
#[derive(Debug, Clone, Logos)]
pub enum Token {
    #[token(r"+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"=")]
    Assign,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(r";")]
    Semicolon,
    #[regex(r"\d+", |lex| lex.slice().parse())]
    UIntNumber(u64),
    #[regex(r"\d+\.\d+", |lex| lex.slice().parse())]
    FloatNumber(f64),
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| String::from(lex.slice()))]
    Ident(String),

    #[regex(r"\s+", logos::skip, priority = 1)]
    Whitespace,

    // Error token
    #[error]
    Error,
}

pub type CalcLexer<'src> = logos::Lexer<'src, Token>;

pub fn new_calc_lexer(s: &str) -> CalcLexer {
    Token::lexer(s)
}
