
use crate::evaluator::*;

pub type Builtin = fn(Vec<Object>) -> Object;


pub fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("wrong number of arguments".to_string());
    }
    match &args[0] {
        Object::Str(x) => Object::Integer(x.len() as i64),
        Object::Array(x) => Object::Integer(x.len() as i64),
        _ => Object::Error("argument to len not supported".to_string())
    }
}

pub fn first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("wrong number of arguments".to_string());
    }
    match &args[0] {
        Object::Array(x) => x[0].clone(),
        _ => Object::Error("argument to first not supported".to_string())
    }
}


pub fn last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("wrong number of arguments".to_string());
    }
    match &args[0] {
        Object::Array(x) => x[x.len()-1].clone(),
        _ => Object::Error("argument to last not supported".to_string())
    }
}


pub fn rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("wrong number of arguments".to_string());
    }
    match &args[0] {
        Object::Array(x) => Object::Array(Rc::new(x[1..].to_vec())),
        _ => Object::Error("argument to rest not supported".to_string())
    }
}

pub fn push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("wrong number of arguments".to_string());
    }
    match &args[0] {
        Object::Array(x) => {
            let mut result = x.as_ref().clone();
            result.push(args[1].clone());
            Object::Array(Rc::new(result))
        }
        _ => Object::Error("argument to len not supported".to_string())
    }
}

pub fn puts(args: Vec<Object>) -> Object {
    for s in args {
        println!("{}", s);
    }
    return Object::Null;
} 