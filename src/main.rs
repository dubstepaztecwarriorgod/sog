mod token;
mod parser;
mod asm;

fn main() {
    let expr = "2 - 3 * (14 / 2)";
    let tokens = token::tokenize(expr.as_bytes());
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse();
    println!("{ast:?}")
}
