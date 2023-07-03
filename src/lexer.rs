use crate::token::{Token, TokenType, lookup_ident};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    pub ch: char,
}

impl Lexer {
    #[allow(dead_code)]
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    #[allow(dead_code)]
    fn read_identifier(&mut self) -> Token {
        println!("Reading identifier");
        let mut ident = "".to_string();
        while self.ch.is_alphabetic() {
            ident = ident + &self.ch.to_string();
            self.read_char();
        }

        lookup_ident(ident)
    }

    #[allow(dead_code)]
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
            '\0' => Token::new(TokenType::EOF, self.ch.to_string()),
            _ if self.ch.is_alphabetic() => {
                return self.read_identifier();
            },
            _ if self.ch.is_numeric() => {
                return self.read_number();
            },
            _ => Token::new(TokenType::ILLEGAL, self.ch.to_string()),
        };
        self.read_char();
        return tok;
    }
    
    #[allow(dead_code)]
    fn skip_whitespace(&mut self) {
        if self.ch == ' ' {
            self.read_char();
        }
    }

    #[allow(dead_code)]
    fn read_number(&mut self) -> Token {
        let mut ident = "".to_string();
        while self.ch.is_numeric() {
            ident = ident + &self.ch.to_string();
            self.read_char();
        }

        Token::new(TokenType::INT, ident)
    }
}
