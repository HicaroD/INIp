use crate::lexer::*;
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::io::{BufRead, BufReader};
use crate::parser::ParserError::*;

type Section = HashMap<String, String>;
type Ini = HashMap<String, Section>;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(char),
    ExpectedAnIdentifier,
    Io(std::io::Error),
}

impl std::error::Error for ParserError {}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> ParserError {
        ParserError::Io(err)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnexpectedToken(t) => write!(f, "Unexpected token: {t}"),
            ExpectedAnIdentifier => write!(f, "Expected an identifier"),
            Io(e) => write!(f, "{e}"),
        }
    }
}

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

    fn add_section(ini_file: &mut Ini, sections: &mut Vec<String>, section_name: &str) {
        sections.push(section_name.to_string());
        if !ini_file.contains_key(section_name) {
            ini_file.insert(section_name.to_string(), HashMap::new());
        }
    }

    fn add_value_to_section<'a>(
        section: &mut Section,
        key: &'a str,
        value: &'a str,
    ) {
        if section.is_empty() || !section.contains_key(key) {
            section.insert(key.to_string(), value.to_string());
        } else if let Some(key_value) = section.get_mut(key) {
            *key_value = value.to_string();
        }
    }

    pub fn parse<S: Into<String>>(file_path: S) -> Result<Ini, ParserError> {
        let file_content = Parser::read_file(file_path)?;
        let tokens = Lexer::tokenize(file_content);
        let mut tokens = tokens.iter();

        let mut ini_file: Ini = HashMap::new();
        let mut sections = Vec::new();

        while let Some(token) = tokens.next() {
            if let Token::OpeningSquareBracket = token {
                if let Some(Token::Identifier(section_name)) = tokens.next() {
                    Parser::add_section(&mut ini_file, &mut sections, section_name);
                } else {
                    return Err(ParserError::ExpectedAnIdentifier);
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

                        if let Some(section) = ini_file.get_mut(last_section_added) {
                            Parser::add_value_to_section(section, key, value);
                        }
                    } else {
                        return Err(ParserError::ExpectedAnIdentifier);
                    }
                }
            } else if let Token::Unknown(t) = token {
                return Err(ParserError::UnexpectedToken(*t));
            }
        }
        Ok(ini_file)
    }
}
