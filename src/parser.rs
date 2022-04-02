use crate::lexer::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

type Ini = HashMap<String, HashMap<String, String>>;

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

    pub fn parse<S: Into<String>>(file_path: S) -> std::io::Result<Ini> {
        let file_content = Parser::read_file(file_path)?;
        let tokens = Lexer::tokenize(file_content);
        let mut tokens = tokens.iter();

        let mut ini_file: Ini = HashMap::new();
        let mut sections = Vec::new();

        while let Some(token) = tokens.next() {
            if let Token::OpeningSquareBracket = token {
                if let Some(Token::Identifier(section_name)) = tokens.next() {
                    println!("It is a section declaration. Value: {section_name}");
                    sections.push(section_name);
                    if !ini_file.contains_key(section_name) {
                        println!("Adding new section: {section_name}");
                        ini_file.insert(section_name.to_string(), HashMap::new());
                    }
                } else {
                    //TODO(Hícaro): Add custom error handling to unexpected token
                    println!("It shouldn't happen. The next token should be an identifier");
                }
            } else if let Token::Identifier(key) = token {
                if let Some(Token::EqualSign) = tokens.next() {
                    if let Some(Token::Identifier(value)) = tokens.next() {
                        let last_section_added = match sections.last() {
                            Some(section_name) => section_name,
                            None => {
                                println!("No sections were added.");
                                std::process::exit(1);
                            }
                        };

                        if let Some(section) = ini_file.get_mut(*last_section_added) {
                            if section.is_empty() || !section.contains_key(key) {
                                section.insert(key.to_string(), value.to_string());
                                println!(
                                    "Add key '{}' and value '{}' on {}",
                                    key, value, *last_section_added
                                );
                            } else if let Some(key_value) = section.get_mut(key) {
                                *key_value = value.to_string();
                                println!("Changing existing key to '{}'", key_value);
                            }
                        }
                    } else {
                        println!("Should be an identifier.")
                    }
                }
            } else if let Token::Unknown(t) = token {
                println!("Unexpected token: '{}'", t);
            }
        }
        Ok(ini_file)
    }
}
