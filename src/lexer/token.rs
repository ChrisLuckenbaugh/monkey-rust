
use std::{ rc::Rc};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String),
    Int(i64),
    Str(Rc<String>),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterick,
    Slash,

    LT,
    GT,
    Equal,
    NotEqual,


    // Delimiters
    Comma, 
    Semicolon,
    Colon,

    LBracket,
    RBracket,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return

}