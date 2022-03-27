use crate::lexer::*;
use std::fs;

pub struct Parser {}

impl Parser {
    pub fn parse<S: Into<String>>(file_path: S) {
        let file = match fs::read_to_string(file_path.into()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to read file. Error: {e}");
                std::process::exit(1);
            }
        };
        let tokens = Lexer::tokenize(file);
        for token in tokens.iter() {
            println!("{:?}", token);
        }
    }
}
