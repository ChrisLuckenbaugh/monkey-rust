use crate::lexer::token::{Token};
use std::fmt;


#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub struct Index(pub usize);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Literal {
    Int(i64),
    Bool(bool),
    Str(String)
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Prefix {
    Plus,
    Minus,
    Not
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    GT,
    LT,
    Equal,
    NotEqual,
    Call,
    Index
}


#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Box<Expr>, Infix, Box<Expr>),
    If(Box<Expr>, Block, Option<Block>),
    Fn(Params, Block),
    Call(Box<Expr>, Args),
    Array(Vec<Expr>),
    IndexExpr(Box<Expr>, Box<Expr>)
}


#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    LetStmt(Ident, Expr),
    ReturnStmt(Expr),
    ExprStmt(Expr),
}

pub type Args = Vec<Expr>;
pub type Params = Vec<Ident>;
pub type Block = Vec<Stmt>;
pub type Program = Block;


