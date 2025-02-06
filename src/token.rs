#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token {
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    LParen,
    RParen
}

macro_rules! token_push {
    ($tokens:expr, $token:expr, $cursor:expr) => {
        {
            $tokens.push($token);
            $cursor += 1;
            continue;
        }
    };
}

pub fn tokenize(input: &[u8]) -> Vec<Token> {
    let mut cursor = 0;
    let mut tokens = vec![];

    while cursor < input.len() {
        if (input[cursor] as char).is_whitespace() {
            cursor += 1;
            continue;
        }
        match input[cursor] {
            b'0'..b'9' => {
                let mut number_buffer = String::new();
                while input[cursor].is_ascii_digit() {
                    number_buffer.push(input[cursor] as char);
                    cursor += 1;
                }
                let num = number_buffer.parse::<i32>().unwrap();
                tokens.push(Token::Num(num));
            },
            b'*' => token_push!(tokens, Token::Mul, cursor),
            b'/' => token_push!(tokens, Token::Div, cursor),
            b'+' => token_push!(tokens, Token::Add, cursor),
            b'-' => token_push!(tokens, Token::Sub, cursor),
            b'^' => token_push!(tokens, Token::Exp, cursor),
            b'(' => token_push!(tokens, Token::LParen, cursor),
            b')' => token_push!(tokens, Token::RParen, cursor),
            _ => panic!("Unexpected token: {}", input[cursor])
        }
    }

    tokens
}