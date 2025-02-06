mod token;
mod parser;
mod asm;

fn main() {
    let expr = "(2 * 3) + 8 / 6 + (2 - 4)";
    let tokens = token::tokenize(expr.as_bytes());
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse();
    println!("{ast:?}");
    let asm = asm::generate_asm(ast);
    println!("{asm}")
}
