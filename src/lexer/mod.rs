pub mod token;

use std::{ rc::Rc};
use token::*;
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: 0 as char
        };
        l.read_char();
        return l;

    }


    fn read_char(&mut self) {
        self.ch = match self.input.chars().nth(self.read_position) {
            Some(c) => c,
            None => 0 as char,
        };
        self.position = self.read_position;
        self.read_position += 1;

    }

    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        return self.input[pos..self.position].to_string();
    }

    fn lookup_ident(ident: String) -> Token {
        return match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident)
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_numer(&mut self) -> i64 {
        let pos = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }
        return self.input[pos..self.position].to_string().parse::<i64>().unwrap();
    }

    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                return ch;
            } else {
                panic!("peeked out of range character")
            }
        }
    }

    fn read_str(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '\"' ||  self.ch == '\u{0000}' { break; }
        } 

        return self.input[position..self.position].to_string();

    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' =>
                if self.peek() == '=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                },
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => 
                if self.peek() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                },
            '/' => Token::Slash,
            '*' => Token::Asterick,
            '<' => Token::LT,
            '>' => Token::GT,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            '"' => {
                Token::Str(Rc::new(self.read_str()))
            },
            '\u{0000}' => Token::EOF,
            _ => {
                if Lexer::is_letter(self.ch) {
                    return Lexer::lookup_ident(
                        self.read_identifier()
                    );
                } else if Lexer::is_digit(self.ch) {
                    return Token::Int(
                        self.read_numer()
                    );
                } else {
                    return Token::Illegal
                }
            }
        };
        self.read_char();
        return tok;
    }

}