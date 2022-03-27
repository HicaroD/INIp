use std::fs;
use std::io::{self, BufRead, BufReader};
use crate::lexer::*;

pub struct Parser {}

impl Parser {
    fn read_file<S: Into<String>>(file_path: S) -> std::io::Result<String> {
        let file = match fs::File::open(file_path.into()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to open file. Error: {e}");
                std::process::exit(1);
            }
        };

        let file_reader = BufReader::new(file);
        let mut file_content = String::new();

        for line in file_reader.lines() {
            let line_content = line?;
            if !line_content.starts_with(";") {
                file_content.push_str(&line_content);
            }
        }
        return Ok(file_content);
    }

    pub fn parse<S: Into<String>>(file_path: S) -> std::io::Result<()> {
        let file_content = Parser::read_file(file_path)?;

        let tokens = Lexer::tokenize(file_content);
        for token in tokens.iter() {
            println!("{:?}", token);
        }
        Ok(())
    }
}
