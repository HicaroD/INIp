mod lexer;
mod parser;

use lexer::*;
use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer() {
        let tokens = Lexer::tokenize("[hi]".to_string());
    }

    #[test]
    fn test_parser() {
        Parser::parse("example.ini");
    }
}
