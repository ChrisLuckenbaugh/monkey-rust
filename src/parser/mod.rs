pub mod ast;
use crate::lexer::{Lexer};
use crate::lexer::token::{Token};
use ast::*;
use std::cmp::Ordering;
use std::error::Error;
use std::{ rc::Rc};


pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    curr: Token,
    peek: Token,
    pub errors: Vec<Box<Error>>
}
#[derive(PartialEq, Debug, Eq, Clone)]
enum Precedence {
    Lowest = 0,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index       
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Precedence) -> Option<Ordering> {
        let x = self.clone() as u8;
        let y = other.clone() as u8;
        return Some(x.cmp(&y));
    }
}


impl<'a> Parser<'a> {

    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let mut p = Parser{l: l, curr: Token::EOF, peek: Token::EOF, errors: vec![]};
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.l.next_token();
    }

    fn curr_is(&self, t: Token) -> bool {
        return self.curr == t;
    }

    fn peek_is(&self, t: &Token) -> bool {
        return std::mem::discriminant(&self.peek) == std::mem::discriminant(&t);
    }

    fn expect_peek(&mut self, token: &Token) -> Result<(), Box<Error>> {
        if self.peek_is(token) {
            self.next_token();
            return Ok(())
        } else {
            return self.peek_error(token);
        }
    }

    fn peek_error(&mut self, t: &Token) -> Result<(), Box<dyn Error>> {
        let msg = format!("expected next token to be {:?}, got {:?} instead", t, self.peek);
        return Err(msg.into());
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, Box<Error>> {

        self.expect_peek(&Token::Ident("".to_string()))?;

        let name = match self.curr.clone() {
            Token::Ident(name) => {
                name
            },
            _ => return Err("expected an ident but got something else!".into())
        };

        self.expect_peek(&Token::Assign)?;

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_is(&Token::Semicolon) {
            self.next_token();
        }

        return Ok(
            Stmt::LetStmt(Ident(name), value)
        );

    }

    fn parse_return_statement(&mut self) -> Result<Stmt, Box<Error>> {
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_is(&Token::Semicolon) {
            self.next_token();
        }

        return Ok(Stmt::ReturnStmt(value));
    }

    fn parse_prefix(&mut self, p: Prefix) -> Result<Expr, Box<Error>> {
        self.next_token();
        let y = self.parse_expression(Precedence::Prefix)?;
        return Ok(Expr::Prefix(p, Box::new(y)));
    }


    fn get_precedence(token: &Token) -> Precedence {
        return match token {
            Token::Equal => Precedence::Equals,
            Token::NotEqual => Precedence::Equals,
            Token::LT => Precedence::LessGreater,
            Token::GT => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterick => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }



    fn peek_precedence(&self) -> Precedence {
        return Parser::get_precedence(&self.peek);
    }

    fn curr_precedence(&self) -> Precedence {
        return Parser::get_precedence(&self.curr);
    }

    fn get_infix(token: &Token) -> Result<Infix, Box<Error>> {
        return match token {
            Token::Minus => Ok(Infix::Minus),
            Token::Plus => Ok(Infix::Plus),
            Token::Slash => Ok(Infix::Divide),
            Token::Asterick => Ok(Infix::Multiply),
            Token::Equal => Ok(Infix::Equal),
            Token::NotEqual => Ok(Infix::NotEqual),
            Token::LT => Ok(Infix::LT),
            Token::GT => Ok(Infix::GT),
            Token::LParen => Ok(Infix::Call),
            Token::LBracket => Ok(Infix::Index),
            _ => Err("".into())
        }
    }

    fn parse_call_args(&mut self) -> Result<Args, Box<Error>> {
        let mut args = vec![];
        if self.peek_is(&Token::RParen) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(
            self.parse_expression(Precedence::Lowest)?
        );

        while self.peek_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            args.push(
                self.parse_expression(Precedence::Lowest)?
            );
        }

        self.expect_peek(&Token::RParen)?;
        return Ok(args);

    }

    fn parse_call_expr(&mut self, function: &Expr) -> Result<Expr, Box<Error>> {
        let args = self.parse_call_args()?;
        return Ok(Expr::Call(Box::new(function.clone()), args));
    }

    fn parse_index_expr(&mut self, left: &Expr) -> Result<Expr, Box<Error>> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(&Token::RBracket)?;
        return Ok(Expr::IndexExpr(Box::new(left.clone()), Box::new(index)));
    }



    fn parse_infix(&mut self, left: &Expr, operator: Infix) -> Result<Expr, Box<Error>>{
        if Infix::Call == operator {
            return self.parse_call_expr(left);
        }

        if Infix::Index == operator {
            return self.parse_index_expr(left)
        }

        let precedence = self.curr_precedence();

        self.next_token();
        let right = self.parse_expression(precedence)?;
        return Ok(Expr::Infix(Box::new(left.clone()), operator, Box::new(right)));
    }

    fn parse_grouped(&mut self) -> Result<Expr, Box<Error>> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(&Token::RParen)?;
        return Ok(exp); 
    }

    fn parse_block(&mut self) -> Result<Block, Box<Error>> {
        let mut block = vec![];
        self.next_token();

        while !self.curr_is(Token::RBrace) && !self.curr_is(Token::EOF) {
            block.push(self.parse_statement()?);
            self.next_token();
        }

        return Ok(block);

    }

    fn parse_if(&mut self) -> Result<Expr, Box<Error>> {


        self.expect_peek(&Token::LParen)?;
        self.next_token();
        // Parse the condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(&Token::RParen)?;
        self.expect_peek(&Token::LBrace)?;


        // Parse the if block
        let consequence = self.parse_block()?;


        // Parse optional else block if available
        let alternative = if self.peek_is(&Token::Else) {
            self.next_token();
            self.expect_peek(&Token::LBrace)?;
            Some(self.parse_block()?)
        } else {
            None
        };
 
        return Ok(Expr::If(Box::new(condition), consequence, alternative)); 
    }

    fn parse_params(&mut self) -> Result<Params, Box<Error>> {
        let mut identifiers = vec![];

        if self.peek_is(&Token::RParen) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();
        match &self.curr {
            Token::Ident(s) => identifiers.push(Ident(s.to_string())),
            _ => return Err("Token is not an identifier".into()),
        }

        while self.peek_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            match &self.curr {
                Token::Ident(s) => identifiers.push(Ident(s.to_string())),
                _ => return Err("Token is not an identifier".into()),
            }
        }

        self.expect_peek(&Token::RParen)?;
        return Ok(identifiers);

    }


    fn parse_function(&mut self) -> Result<Expr, Box<Error>> {
        self.expect_peek(&Token::LParen)?;
        let parameters = self.parse_params()?;
        self.expect_peek(&Token::LBrace)?;
        let block = self.parse_block()?;
        return Ok(Expr::Fn(Rc::new(parameters), Rc::new(block))); 
    }

    fn get_prefix(&mut self, token: &Token) -> Result<Expr, Box<Error>> {
        match token {
            Token::Ident(x) => Ok(Expr::IdentExpr(Ident(x.to_string()))),
            Token::Int(x) => Ok(Expr::LiteralExpr(Literal::Int(*x))),
            Token::True => Ok(Expr::LiteralExpr(Literal::Bool(true))),
            Token::False => Ok(Expr::LiteralExpr(Literal::Bool(false))),
            Token::Str(x) => Ok(Expr::LiteralExpr(Literal::Str(Rc::clone(x)))),
            Token::Minus => self.parse_prefix(Prefix::Minus),
            Token::Plus => self.parse_prefix(Prefix::Plus),
            Token::Bang => self.parse_prefix(Prefix::Not),
            Token::LParen => self.parse_grouped(),
            Token::If => self.parse_if(),
            Token::Function => self.parse_function(),
            Token::LBracket => self.parse_array(),
            Token::LBrace => self.parse_hash_literal(),
            _ => {
                Err(format!("No prefix defined for {:?}", token).into())
            }
        }
    }

    fn parse_hash_literal(&mut self) -> Result<Expr, Box<Error>> {
        let mut list = vec![];
        while !self.peek_is(&Token::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            self.expect_peek(&Token::Colon)?;
            self.next_token();

            let value = self.parse_expression(Precedence::Lowest)?;
            list.push((key,value));

            if !self.peek_is(&Token::RBrace) {
                self.expect_peek(&Token::Comma)?;
            }
        }
        
        self.expect_peek(&Token::RBrace)?;
        return Ok(Expr::LiteralExpr(Literal::Hash(list)));

    }

    fn parse_array(&mut self) -> Result<Expr, Box<Error>> {
        let elements = self.parse_expression_list(&Token::RBracket)?;
        return Ok(Expr::Array(elements))
    }



    fn parse_expression_list(&mut self, end: &Token) -> Result<Vec<Expr>, Box<Error>> {
        let mut list = vec![];
        if self.peek_is(end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_peek(end)?;
        return Ok(list);
    }

    fn parse_expression(&mut self, precedece: Precedence) -> Result<Expr, Box<Error>>  {

        let prefix = self.get_prefix(&self.curr.clone())?;
        let mut left = prefix;
        while !self.peek_is(&Token::Semicolon) && precedece < self.peek_precedence() {
            let infix = Parser::get_infix(&self.peek)?;
            self.next_token();
            left = self.parse_infix(&left,infix)?;
        }
        return Ok(left)
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, Box<Error>> {
        let exp = self.parse_expression(Precedence::Lowest)?;

        if self.peek_is(&Token::Semicolon) {
            self.next_token();
        }

        return Ok(Stmt::ExprStmt(exp))
    }


    fn parse_statement(&mut self) -> Result<Stmt, Box<Error>> {
        return match self.curr {
            Token::Let => Ok(self.parse_let_statement()?),
            Token::Return => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?)
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, &Vec<Box<Error>>> {
        let mut program: Program = Vec::new();
        while self.curr != Token::EOF {
            match self.parse_statement() {
                Ok(x) => program.push(x),
                Err(x) => self.errors.push(x)
            }
            self.next_token();
        }

        if self.errors.len() != 0 {
            return Err(&self.errors)
        } else {
            return Ok(program);
        }

    }
}



