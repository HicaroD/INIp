mod lexer;
mod parser;

use lexer::*;
use parser::*;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let tokens = Lexer::tokenize("[hi]".to_string());
        let expected_tokens = vec![
            Token::OpeningSquareBracket,
            Token::Identifier("hi".to_string()),
            Token::ClosingSquareBracket,
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_parser() {
        let parsed_file = Parser::parse("example.ini").unwrap();
        let mut expected_result = HashMap::new();
        expected_result.insert(
            "Hicaro".to_string(),
            HashMap::from([("value".to_string(), "10".to_string())]),
        );
        expected_result.insert(
            "section".to_string(),
            HashMap::from([
                ("something".to_string(), "30".to_string()),
                ("name".to_string(), "Hicaro".to_string()),
            ]),
        );

        assert_eq!(parsed_file, expected_result);
    }
}
