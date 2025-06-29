mod lexer;
mod parser;

use lexer::{Lexer};

fn main() {
    let s = "{ \"foo\": \"bar\", \"abc\": { \"def\": 42 } }";
    let mut l = Lexer::new(s);
    let t = l.tokenize();

    println!("{:?}", t);
}
