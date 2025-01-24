mod token;
mod parser;

fn main() {
    let expr = "2 - 3 * (14 / 2)";
    let tokens = token::tokenize(expr.as_bytes());
    println!("{tokens:?}");
}
