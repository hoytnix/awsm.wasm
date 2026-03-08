use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    List(Vec<Expr>),
    Map(HashMap<String, Expr>),
    Symbol(String),
    Keyword(String),
    String(String),
    Selector(String),
    Pointer(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen, RParen, LBrace, RBrace,
    Symbol(String), Keyword(String), String(String),
    Selector(String), Pointer(String),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { chars: input.chars().peekable() }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\n' | '\t' | '\r' => { self.chars.next(); }
                ';' => {
                    while let Some(&ch) = self.chars.peek() {
                        if ch == '\n' { break; }
                        self.chars.next();
                    }
                }
                '(' => { tokens.push(Token::LParen); self.chars.next(); }
                ')' => { tokens.push(Token::RParen); self.chars.next(); }
                '{' => { tokens.push(Token::LBrace); self.chars.next(); }
                '}' => { tokens.push(Token::RBrace); self.chars.next(); }
                '"' => tokens.push(Token::String(self.read_string()?)),
                '$' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'[') {
                        tokens.push(Token::Selector(self.read_selector()?));
                    } else { return Err("Expected '[' after '$'".to_string()); }
                }
                ':' => {
                    self.chars.next();
                    tokens.push(Token::Keyword(self.read_word()));
                }
                '@' => {
                    self.chars.next();
                    tokens.push(Token::Pointer(self.read_word()));
                }
                _ => {
                    let word = self.read_word();
                    if !word.is_empty() { tokens.push(Token::Symbol(word)); }
                }
            }
        }
        Ok(tokens)
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.chars.next();
        let mut res = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' { self.chars.next(); return Ok(res); }
            res.push(self.chars.next().unwrap());
        }
        Err("Unterminated string".to_string())
    }

    fn read_selector(&mut self) -> Result<String, String> {
        self.chars.next();
        let mut res = String::new();
        let mut depth = 1;
        while let Some(&c) = self.chars.peek() {
            if c == '[' { depth += 1; }
            if c == ']' {
                depth -= 1;
                if depth == 0 { self.chars.next(); return Ok(res.trim().to_string()); }
            }
            res.push(self.chars.next().unwrap());
        }
        Err("Unterminated selector".to_string())
    }

    fn read_word(&mut self) -> String {
        let mut res = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == ';' { break; }
            res.push(self.chars.next().unwrap());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_errors() {
        assert!(Lexer::new("\"unterminated").tokenize().is_err());
        assert!(Lexer::new("$").tokenize().is_err());
        assert!(Lexer::new("$[unterminated").tokenize().is_err());
        
        let mut lexer = Lexer::new("$[foo[bar]]");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Selector("foo[bar]".to_string()));
        
        let mut lexer = Lexer::new("; comment\n:keyword @pointer symbol");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Keyword("keyword".to_string()));
        assert_eq!(tokens[1], Token::Pointer("pointer".to_string()));
        assert_eq!(tokens[2], Token::Symbol("symbol".to_string()));
    }
}
