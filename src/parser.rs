use crate::lexer::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

type Ini = HashMap<String, Option<HashMap<String, String>>>;

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

        let mut ini_file: Ini = HashMap::new();
        let mut sections = Vec::new();

        while let Some(token) = tokens.next() {
            match token {
                Token::OpeningSquareBracket => {
                    if let Some(Token::Identifier(section_name)) = tokens.next() {
                        // TODO(Hícaro): Add new section to Ini HashMap
                        println!("It is a new section declaration. Value: {section_name}");
                        sections.push(section_name);
                        if !ini_file.contains_key(section_name) {
                            ini_file.insert(section_name.to_string(), None);
                        }
                    } else {
                        //TODO(Hícaro): Add custom error handling to unexpected token
                        println!("It shouldn't happen. The next token should be an identifier");
                    }
                }
                Token::ClosingSquareBracket => println!("Closing square bracket"),
                Token::Hash => println!("Hash"),
                Token::EqualSign => println!("Equal sign"),
                Token::Identifier(key) => {
                    if let Some(Token::EqualSign) = tokens.next() {
                        // Cheque se o próximo é também um identifier
                        if let Some(Token::Identifier(value)) = tokens.next() {
                            // TODO(Hícaro): Implement key-value pair on section

                            // 1. Se as seções forem vazias, então eu tenho um key-value pair fora de
                            // uma seção. Logo, isso não deve ser válido em meu INI file.
                            
                            // 2. Se eu tentar inserir em seção existente que aponta para None, eu
                            // precisarei criar uma nova HashMap dentro da key da seção.
                            
                            // 3. Se eu tentar inserir em uma seção existente que aponta para uma
                            // HashMap existente, eu preciso inserir meu valor nessa nova HashMap
                            println!("Add new key-value to {:?}", sections.last());
                        } else {
                            println!("Should be an identifier.")
                        }
                    } 
                }
                Token::Unknown(token) => println!("Unknown token: {:?}", token),
            }
        }
        Ok(())
    }
}
