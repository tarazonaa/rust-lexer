use lexer::Lexer;
use token::Token;

mod token;
mod lexer;

fn main() {
        let input = "let five = 5;".to_string();
        let mut l = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while l.ch != '\0' {
            tokens.push(l.next_token())
        }
    
}

#[cfg(test)]
mod tests {
    use crate::{token::TokenType, lexer::Lexer};

    use super::*;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let mut l = lexer::Lexer::new(input);
        assert_eq!(l.next_token(), Token::new(TokenType::ASSIGN, "=".to_string()));
    }

    #[test]
    fn test_all() {
        let input = "=+(){},;".to_string();
        let mut l = lexer::Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while l.ch != '\0' {
            tokens.push(l.next_token());
        }
        assert_eq!(tokens, 
                   [Token::new(TokenType::ASSIGN, "=".to_string()),
                    Token::new(TokenType::PLUS, "+".to_string()),
                    Token::new(TokenType::LPAREN, "(".to_string()),
                    Token::new(TokenType::RPAREN, ")".to_string()),
                    Token::new(TokenType::LBRACE, "{".to_string()),
                    Token::new(TokenType::RBRACE, "}".to_string()),
                    Token::new(TokenType::COMMA, ",".to_string()),
                    Token::new(TokenType::SEMICOLON, ";".to_string())]);
    }

    #[test]
    fn test_monkey() {
        let input = "let five = 5;".to_string();
        let mut l = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while l.ch != '\0' {
            tokens.push(l.next_token())
        }
        assert_eq!(tokens,
                   [Token::new(TokenType::LET, "let".to_string()),
                   Token::new(TokenType::IDENT, "five".to_string()),
                   Token::new(TokenType::ASSIGN, "=".to_string()),
                   Token::new(TokenType::INT, "5".to_string()),
                   Token::new(TokenType::SEMICOLON, ";".to_string())])
    }
}
