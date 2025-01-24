use crate::token::Token;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp
}

pub enum Node {
    Number(i32), 
    Operator {
        operation: Operation,
        left: Box<Node>,
        right: Box<Node>
    }
}

fn precedence(op: Operation) -> i32 {
    match op {
        Operation::Add | Operation::Sub => 1,
        Operation::Div | Operation::Mul => 2,
        Operation::Exp => 3
    }
}