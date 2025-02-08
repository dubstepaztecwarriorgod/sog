mod token;
mod parser;
mod asm;
mod cli;

fn main() {
    let args = cli::Args::new();
    let expression = args.expression();

    let tokens = token::tokenize(expression.as_bytes());
    let ast = parser::Parser::new(tokens.clone()).parse();
    let asm = asm::generate_asm(ast.clone());

    if args.is_debug() {
        println!("Tokens list:\n{:?}\n", tokens);
        println!("Syntax tree:\n{:?}\n", ast);
        println!("Assembly:{}", asm)
    }
}
