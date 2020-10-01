extern crate interpreter;

use interpreter::lexer::token::{Token};
use interpreter::lexer::{Lexer};

#[test]
fn lex_next_token() {
    let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        \"foobar\"
        \"foo bar\"
    ";


    let expected = vec![
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,

        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,

        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,

        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LParen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RParen,
        Token::Semicolon,

        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterick,
        Token::Int(5),
        Token::Semicolon,

        Token::Int(5),
        Token::LT,
        Token::Int(10),
        Token::GT,
        Token::Int(5),
        Token::Semicolon,

        Token::If,
        Token::LParen,
        Token::Int(5),
        Token::LT,
        Token::Int(10),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,


        Token::Int(10),
        Token::Equal,
        Token::Int(10),
        Token::Semicolon,

        Token::Int(10),
        Token::NotEqual,
        Token::Int(9),
        Token::Semicolon,

        Token::Str("foobar".to_string()),
        Token::Str("foo bar".to_string()),


        Token::EOF,
    ];

    let mut l = Lexer::new(input);
    for tt in expected {
        let tok = l.next_token();
        println!("{:?}", tok);
        assert_eq!(tt, tok);
    }

}