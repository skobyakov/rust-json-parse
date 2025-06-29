use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Token {
    RightBrace,
    LeftBrace,
    Colon,
    String(String),
    Number(u64),
    Comma,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    length: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable(), position: 0, length: input.len() }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.length {
            let c = self.input.peek();

            let token = match c {
                None => break,
                Some('{') => { self.input.next(); Token::LeftBrace },
                Some('}') => { self.input.next(); Token::RightBrace },
                Some(':') => { self.input.next(); Token::Colon },
                Some('"') => self.read_string(),
                Some('1'..='9') => self.read_number(),
                Some(',') => { self.input.next(); Token::Comma },
                _ => { self.input.next(); continue}
            };

            tokens.push(token);
        }

        tokens
    }

    fn read_string(&mut self) -> Token {
        self.input.next(); // Read first quote
        let mut c = *self.input.peek().expect("Unexpected EOF");
        let mut res = String::new();

        while c != '"' {
            res.push(c);
            self.input.next();

            c = *self.input.peek().expect("Unexpected EOF");
        }

        self.input.next();

        Token::String(res)
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut c = *self.input.peek().expect("Unexpected EOF");

        while c.is_digit(10) {
            num_str.push(c);
            self.input.next();
            c = *self.input.peek().expect("Unexpected EOF");
        }

        let res = num_str.parse::<u64>().expect("Invalid number");

        Token::Number(res)
    }
}