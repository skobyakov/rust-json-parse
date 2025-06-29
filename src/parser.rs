use std::collections::HashMap;
use std::iter::Peekable;
use crate::lexer::Token;

#[derive(Debug)]
pub enum JSONParserState {
    Object(HashMap<String, JSONParserState>),
    Array(Vec<JSONParserState>),
    String(String),
    Number(u64),
    Boolean(bool),
    Null,
}

pub struct JSONParser {
    state: JSONParserState,
    input: Peekable<std::vec::IntoIter<Token>>,
}

impl JSONParser {
    pub fn new(input: Vec<Token>) -> JSONParser {
        JSONParser {
            state: JSONParserState::Object(HashMap::new()),
            input: input.into_iter().peekable()
        }
    }

    pub fn get_state(&self) -> &JSONParserState {
        &self.state
    }

    pub fn parse(&mut self) {
        match self.input.next() {
            Some(Token::LeftBrace) => {
                self.state = JSONParserState::Object(self.parse_object());
            }
            _ => panic!("Unexpected start of object")
        }
    }
    
    fn parse_object(&mut self) -> HashMap<String, JSONParserState> {
        let mut obj = HashMap::new();

        while match self.input.peek().unwrap() {
            Token::RightBrace => false,
            _ => true,
        } {
            match self.input.peek().unwrap() {
                Token::Comma => {
                    self.input.next().unwrap();
                    continue
                },
                _ => {},
            }

            let key = match self.input.next().unwrap() {
                Token::String(s) => {
                    s.clone()
                },
                _ => panic!("Expected string key"),
            };

            match self.input.next().unwrap() {
                Token::Colon => {},
                _ => panic!("Unexpected token"),
            }

            let value = self.parse_value();

            obj.insert(key, value);
        }

        obj
    }

    fn parse_value(&mut self) -> JSONParserState {
        JSONParserState::Number(42)
    }
}