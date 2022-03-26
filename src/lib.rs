#[derive(Debug)]
enum Token {
    OpenSquareBracket,
    ClosingSquareBracket,
    Hash,
    EqualSign,
    Identifier(String),
    Unknown,
}

struct Lexer {}

impl Lexer {
    fn tokenize(source_file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for character in source_file.chars() {
            match character {
                '='   => tokens.push(Token::EqualSign),
                '['   => tokens.push(Token::OpenSquareBracket),
                ']'   => tokens.push(Token::ClosingSquareBracket),
                token => tokens.push(Token::Unknown),
            }
        }

        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source_file = String::from("[something]\nname=hicaro\n");
        let tokenized_file = Lexer::tokenize(source_file);
        for token in tokenized_file.iter() {
            println!("{:?}", token)
        }
    }
}
