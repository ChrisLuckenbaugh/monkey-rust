
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String),
    Int(i64),
    Str(String),

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