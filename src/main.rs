mod lexer;
mod parser;

use lexer::{Lexer};
use parser::{JSONParser};

fn main() {
    let s = "{ \"foo\": \"bar\", \"abc\": { \"def\": 42 } }";
    let mut l = Lexer::new(s);
    let t = l.tokenize();

    println!("{:?}", t);

    let mut p = JSONParser::new(t);
    p.parse();

    let s = p.get_state();

    println!("{:?}", s);
}
