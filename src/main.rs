mod token;
mod parser;
mod asm;
mod cli;

use std::fs::{self, File};

fn main() {
    let args = cli::Args::new();

    let tokens = token::tokenize(args.expression().as_bytes());
    let ast = parser::Parser::new(tokens.clone()).parse();
    let asm = asm::generate_asm(ast.clone());

    if args.is_debug() {
        println!("Tokens list:\n{:?}\n", tokens);
        println!("Syntax tree:\n{:?}\n", ast);
        println!("Assembly:{}", &asm)
    }

    let path = format!("{}.asm", args.output());
    File::create(&path).unwrap();
    fs::write(path, asm).unwrap();
}
