mod lexer;
use lexer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokens = Lexer::tokenize("[hello]".to_string());
        for token in tokens.iter() {
            println!("{:?}", token)
        }
    }
}
