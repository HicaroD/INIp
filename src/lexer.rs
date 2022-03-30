#[derive(Debug)]
pub enum Token {
    OpeningSquareBracket,
    ClosingSquareBracket,
    Hash,
    EqualSign,
    Identifier(String),
    Unknown(char),
}

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(source_file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut file = source_file.chars().peekable();

        while let Some(token) = file.next() {
            match token {
                '=' => tokens.push(Token::EqualSign),
                '[' => tokens.push(Token::OpeningSquareBracket),
                ']' => tokens.push(Token::ClosingSquareBracket),
                '#' => tokens.push(Token::Hash),
                token => {
                    let mut identifier = String::new();
                    if token.is_alphanumeric() {
                        identifier.push(token);
                        while let Some(t) = file.next_if(|x| x.is_alphanumeric()) {
                            identifier.push(t);
                        }
                        tokens.push(Token::Identifier(identifier));
                    } else if !token.is_whitespace() {
                        tokens.push(Token::Unknown(token));
                    }
                }
            }
        }
        tokens
    }
}
