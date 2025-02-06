use crate::token::Token;

#[derive(Debug)]
#[allow(unused)]
pub enum Expr {
    Num(i32),
    Op {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = self.parse_term();
        while self.peek().map_or(false, |t| matches!(t, Token::Add | Token::Sub)) {
            let op = self.next().unwrap();
            let right = self.parse_term();
            expr = Expr::Op {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while self.peek().map_or(false, |t| matches!(t, Token::Mul | Token::Div)) {
            let op = self.next().unwrap();
            let right = self.parse_factor();
            expr = Expr::Op {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while self.peek() == Some(&Token::Exp) {
            let op = self.next().unwrap();
            let right = self.parse_factor();
            expr = Expr::Op {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next() {
            Some(Token::Num(n)) => Expr::Num(n),
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                if self.next() != Some(Token::RParen) {
                    panic!("Expected closing parentheses");
                }
                expr
            }
            _ => panic!("Unrecognized token"),
        }
    }
    

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn next(&mut self) -> Option<Token> {
        if self.cursor < self.tokens.len() {
            let token = self.tokens[self.cursor].clone();
            self.cursor += 1;
            Some(token)
        } else {
            None
        }
    }
}

