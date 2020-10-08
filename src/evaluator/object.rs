use std::fmt;
use std::{rc::Rc};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::parser::ast::*;
use crate::evaluator::builtins::Builtin;
use crate::evaluator::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Ret(Box<Object>),
    Boolean(bool),
    Str(Rc<String>),
    Null,
    Error(String),
    Closure(Rc<Params>, Rc<Block>, Environment),
    Builtin(Builtin),
    Array(Rc<Vec<Object>>),
    Hash(HashMap<Object,Object>)
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Object::Integer(ref i) => i.hash(state),
            Object::Boolean(ref b) => b.hash(state),
            Object::Str(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(x) => write!(f, "{}", x),
            Object::Boolean(x) => write!(f, "{}", x),
            Object::Null => write!(f, "null"),
            Object::Error(s) => write!(f, "ERROR {}", s),
            Object::Closure(params, block, env) => write!(f, "({:?}) => {:?}", params, block),
            Object::Str(x) => write!(f, "{}", x),
            Object::Ret(x) => write!(f, "{}", *x),
            Object::Builtin(x) => write!(f, "{:?}", x),
            Object::Array(x) => write!(f, "[{}]", x.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Object::Hash(x) => write!(f, "{:?}", x)
        }
    }
}