use std::collections::HashMap;
use crate::lexer::Token;

enum JSONParserState {
    Object(HashMap<String, JSONParserState>),
    Array(Vec<JSONParserState>),
    String(String),
    Number(u64),
    Boolean(bool),
    Null,
}

struct JSONParser {
    state: JSONParserState,
    input: Vec<Token>,
}

impl JSONParser {
    fn new(input: Vec<Token>) -> JSONParser {
        JSONParser{state: JSONParserState::Object(HashMap::new()), input}
    }

    fn parse(&mut self) {
    
    }


}