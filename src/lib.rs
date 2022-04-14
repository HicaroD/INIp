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
        let tokens = Lexer::tokenize("[hi]name=\'Hicaro\'".to_string());
        let expected_tokens = vec![
            Token::OpeningSquareBracket,
            Token::SectionName("hi".to_string()),
            Token::ClosingSquareBracket,
            Token::Key("name".to_string()),
            Token::EqualSign,
            Token::Value("Hicaro".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_parser_on_valid_file() {
        let parsed_file = Parser::parse("examples/valid/example.ini").unwrap();

        let mut expected_result = HashMap::new();
        expected_result.insert(
            "Hicaro".to_string(),
            HashMap::from([("value".to_string(), "10".to_string())]),
        );
        expected_result.insert(
            "Important section".to_string(),
            HashMap::from([
                ("something".to_string(), "30".to_string()),
                ("name".to_string(), "Hicaro Dânrlley".to_string()),
            ]),
        );
        assert_eq!(parsed_file, expected_result);
    }

    #[test]
    fn test_double_square_bracket_on_section_declaration() {
        let parsed_file = Parser::parse("examples/invalid/example.ini");
        assert!(parsed_file.is_err());
    }

    #[test]
    fn test_invalid_equal_sign_before_key_declaration() {
        let parsed_file = Parser::parse("examples/invalid/example1.ini");
        assert!(parsed_file.is_err());
    }

    #[test]
    fn test_value_with_quotes() {
        let parsed_file = Parser::parse("examples/valid/example2.ini");
        let mut expected_result = HashMap::new();
        expected_result.insert(
            "section".to_string(),
            HashMap::from([("key".to_string(), ("value =".to_string()))]),
        );
        expected_result.insert(
            "another section".to_string(),
            HashMap::from([("key".to_string(), "hello".to_string())]),
        );
        assert_eq!(parsed_file.unwrap(), expected_result);
    }

    #[test]
    fn test_invalid_key_name_with_spaced_name() {
        let parsed_file = Parser::parse("examples/invalid/example2.ini");
        assert!(parsed_file.is_err());
    }

    #[test]
    fn test_disable_entry_recognition() {
        let parsed_file = Parser::parse("examples/valid/example3.ini");
        let mut expected_result = HashMap::new();
        expected_result.insert("section".to_string(), HashMap::new());
        assert_eq!(parsed_file.unwrap(), expected_result);
    }

    #[test]
    fn test_key_name_with_underscore() {
        let parsed_file = Parser::parse("examples/valid/example4.ini");
        assert!(!parsed_file.is_err());
    }

    #[test]
    fn test_key_values() {
        let parsed_file = Parser::parse("examples/valid/example4.ini").unwrap();
        assert_eq!(parsed_file["section"]["full_name"], "Hicaro".to_string());
        assert_eq!(
            parsed_file["section"]["another_key"],
            "Something here".to_string()
        );
    }

    #[test]
    fn test_empty_sections() {
        let parsed_file = Parser::parse("examples/valid/example5.ini");
        let mut expected_result = HashMap::new();
        expected_result.insert("first section".to_string(), HashMap::new());
        expected_result.insert("second section".to_string(), HashMap::new());

        assert_eq!(parsed_file.unwrap(), expected_result);
    }
}
