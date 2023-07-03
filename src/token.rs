// Purpose: Defines the Token struct and TokenType type.

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

#[allow(dead_code)]
pub fn lookup_ident(ident: String) -> Token {
    let keywords: HashMap<String, TokenType> = HashMap::from([
        ("fn".to_string(), TokenType::FUNCTION),
        ("let".to_string(), TokenType::LET),
    ]);
    if keywords.contains_key(&ident) {
        let token_type = keywords.get(&ident).unwrap();
        let literal = ident;
        return Token::new(token_type.clone(), literal);
    }

    Token::new(TokenType::IDENT, ident)
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
