mod token;
mod lexer;

fn lex(input: String) {
    let mut l = lexer::Lexer::new(input);
    loop {
        let tok = l.read_char();
        if tok == token::Token::Eof {
            break;
        }
        println!("{:?}", tok);
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let l = lexer::Lexer::new(input);
        assert_eq!(next_token(input), Token::Assign);
    }
}
