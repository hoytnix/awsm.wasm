use std::collections::HashMap;
use crate::lexer::{Expr, Token};

pub struct Parser {
    pub tokens: std::vec::IntoIter<Token>,
    pub current: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter();
        let current = iter.next();
        Parser { tokens: iter, current }
    }

    fn advance(&mut self) { self.current = self.tokens.next(); }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let token = self.current.clone().ok_or("Unexpected EOF")?;
        match token {
            Token::LParen => {
                self.advance();
                let mut list = Vec::new();
                while self.current.is_some() && self.current != Some(Token::RParen) {
                    list.push(self.parse_expr()?);
                }
                self.advance(); // Consume RParen
                Ok(Expr::List(list))
            }
            Token::LBrace => {
                self.advance();
                let mut map = HashMap::new();
                while self.current.is_some() && self.current != Some(Token::RBrace) {
                    let key = match self.parse_expr()? {
                        Expr::Keyword(k) | Expr::Symbol(k) | Expr::String(k) => k,
                        _ => return Err("Map keys must be strings/keywords".to_string()),
                    };
                    map.insert(key, self.parse_expr()?);
                }
                self.advance(); // Consume RBrace
                Ok(Expr::Map(map))
            }
            Token::Symbol(s) => {
                self.advance();
                match s.as_str() {
                    "true" => Ok(Expr::Bool(true)),
                    "false" => Ok(Expr::Bool(false)),
                    _ => Ok(Expr::Symbol(s)),
                }
            }
            Token::Keyword(k) => { self.advance(); Ok(Expr::Keyword(k)) }
            Token::String(s) => { self.advance(); Ok(Expr::String(s)) }
            Token::Selector(s) => { self.advance(); Ok(Expr::Selector(s)) }
            Token::Pointer(p) => { self.advance(); Ok(Expr::Pointer(p)) }
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_coverage() {
        // test unexpected token
        let mut parser = Parser::new(vec![Token::RParen]);
        assert!(parser.parse_expr().is_err());

        // test invalid map key
        let mut parser = Parser::new(vec![Token::LBrace, Token::LParen, Token::RParen, Token::RBrace]);
        assert!(parser.parse_expr().is_err());

        // test parse selector
        let mut parser = Parser::new(vec![Token::Selector("sel".to_string())]);
        assert_eq!(parser.parse_expr().unwrap(), Expr::Selector("sel".to_string()));
    }
}
