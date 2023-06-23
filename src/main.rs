mod token;
mod lexer;

fn lex(input: String) {
    let mut l = lexer::Lexer::new(input);
    loop {
        let tok = l.read_char();
        println!("{:?}", tok);
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let mut l = lexer::Lexer::new(input);
        assert_eq!(l.next_token(), Token::new(TokenType::ASSIGN, "=".to_string()));
    }
}
