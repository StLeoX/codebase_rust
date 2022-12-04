//! parser
use super::lexer::{new_calc_lexer, CalcLexer, Token};

//////////
// Node Type
pub enum NodeType {
    Plus,
    Minus,
    Mul,
    Div,
    Number,
}

// ParseTree
pub struct Tree {
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
    pub value: Option<f64>,
    pub ty: NodeType,
}

impl Tree {
    fn new(left: Tree, right: Tree, value: f64, ty: NodeType) -> Tree {
        Tree {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            value: Some(value),
            ty: ty,
        }
    }

    fn dummy(value: f64, ty: NodeType) -> Tree {
        Tree {
            left: None,
            right: None,
            value: Some(value),
            ty: ty,
        }
    }
}
//////////

pub struct CalcParser<'a> {
    pub lexer: CalcLexer<'static>,
    pub current_token: &'a mut Token,
}

impl<'a> CalcParser<'a> {
    pub fn new(input: &'static str) -> CalcParser<'a> {
        let mut lexer = new_calc_lexer(&input);
        CalcParser {
            lexer: lexer,
            current_token: &mut lexer.next().unwrap(),
        }
    }

    fn eat(&self) {
        *self.current_token = self.lexer.next().unwrap();
    }

    // number
    fn factor(&self) -> Tree {
        match self.current_token {
            Token::UIntNumber(x) => {
                self.eat();
                Tree::dummy(*x as f64, NodeType::Number)
            }
            Token::FloatNumber(x) => {
                self.eat();
                Tree::dummy(*x, NodeType::Number)
            }
            Token::LParen => {
                self.eat();
                let tree = self.expr();
                self.eat();
                tree
            }
            _ => panic!("factor error"),
        }
    }

    // mul or div
    fn term(&self) -> Tree {
        let tree = self.factor();
        match self.current_token {
            Token::Mul => {
                self.eat();
                Tree::new(tree, self.factor(), 0f64, NodeType::Mul)
            }
            Token::Div => {
                self.eat();
                Tree::new(tree, self.factor(), 0f64, NodeType::Div)
            }
            _ => panic!("term error"),
        }
    }

    // plus or minus
    fn expr(&self) -> Tree {
        let tree = self.factor();
        match self.current_token {
            Token::Plus => {
                self.eat();
                Tree::new(tree, self.term(), 0f64, NodeType::Plus)
            }
            Token::Minus => {
                self.eat();
                Tree::new(tree, self.term(), 0f64, NodeType::Minus)
            }
            _ => panic!("expr error"),
        }
    }

    pub fn parse(&self) -> Tree {
        self.expr()
    }
}
