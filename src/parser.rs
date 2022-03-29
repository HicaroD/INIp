use crate::lexer::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

type INI = HashMap<String, Option<HashMap<String, String>>>;

pub struct Parser {}

impl Parser {
    fn read_file<S: Into<String>>(file_path: S) -> std::io::Result<String> {
        let file = fs::File::open(file_path.into())?;
        let file_reader = BufReader::new(file);
        let mut file_content = String::new();

        for line in file_reader.lines() {
            let line_content = line?;
            if !line_content.starts_with(';') {
                file_content.push_str(&line_content);
            }
        }
        Ok(file_content)
    }

    pub fn parse<S: Into<String>>(file_path: S) -> std::io::Result<()> {
        let file_content = Parser::read_file(file_path)?;
        let tokens = Lexer::tokenize(file_content);
        let mut tokens = tokens.iter();

        let mut ini_file: INI = HashMap::new();

        while let Some(token) = tokens.next() {
            match token {
                Token::OpeningSquareBracket => {
                    if let Some(Token::Identifier(ident)) = tokens.next() {
                        // TODO(Hícaro): Add new section to INI HashMap
                        println!("It is an identifier");
                    } else {
                        //TODO(Hícaro): Add custom error handling to unexpected token
                        println!("It shouldn't happen. The next token should be an identifier");
                    }
                }
                Token::ClosingSquareBracket => println!("Closing square bracket"),
                Token::Hash => println!("Hash"),
                Token::EqualSign => println!("Equal sign"),
                Token::Identifier(ident) => println!("Identifier: {ident}"),
                Token::Unknown(unknown_token) => println!("Unknown token: {unknown_token}"),
            }
        }
        Ok(())
    }
}
