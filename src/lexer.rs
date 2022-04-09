#[derive(Debug, PartialEq)]
pub enum Token {
    OpeningSquareBracket,
    ClosingSquareBracket,
    EqualSign,
    Value(String),
    Key(String),
    Unknown(char),
    SectionName(String),
}

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(source_file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut file = source_file.chars().peekable();

        while let Some(token) = file.next() {
            match token {
                '=' => tokens.push(Token::EqualSign),
                '[' => {
                    tokens.push(Token::OpeningSquareBracket);

                    let mut section_name = String::new();
                    while let Some(t) = file.next_if(|x| *x != ']') {
                        section_name.push(t);
                    }
                    tokens.push(Token::SectionName(section_name));
                },
                ']' => tokens.push(Token::ClosingSquareBracket),
                '\'' | '\"' => {
                    let mut value = String::new();
                    while let Some(t) = file.next() {
                        if t == '\'' || t == '\"' {
                            break;
                        }
                        value.push(t);
                    }
                    tokens.push(Token::Value(value));
                }
                token => {
                    let mut identifier = String::new();
                    if token.is_alphanumeric() {
                        identifier.push(token);
                        while let Some(t) =
                            file.next_if(|x| x.is_alphanumeric() || *x == '_')
                        {
                            identifier.push(t);
                        }
                        tokens.push(Token::Key(identifier.trim_end().to_string()));
                    } else if !token.is_whitespace() {
                        tokens.push(Token::Unknown(token));
                    }
                }
            }
        }
        tokens
    }
}
