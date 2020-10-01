use crate::parser::ast::*;
use std::fmt;
use std::{cell::RefCell, collections::HashMap};

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(i64),
    Ret(Box<Object>),
    Boolean(bool),
    Str(String),
    Null,
    Error(String),
    Closure(Params, Block, Environment),
    Builtin(String),
    Array(Vec<Object>)
}


pub type Environment = std::collections::HashMap<Ident,RefCell<Object>>;



impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(x) => write!(f, "{}", x),
            Object::Boolean(x) => write!(f, "{}", x),
            Object::Null => write!(f, "null"),
            Object::Error(s) => write!(f, "ERROR {}", s),
            Object::Closure(params, block, env) => write!(f, "({:?}) => {:?}", params, block),
            Object::Str(x) => write!(f, "\"{}\"", x),
            Object::Ret(x) => write!(f, "{}", *x),
            Object::Builtin(x) => write!(f, "{}", x),
            Object::Array(x) => write!(f, "[{}]", x.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
        }
    }
}


pub fn evaluate_statement(statement: Stmt, env: &mut Environment) -> Object {
    return match statement {
        Stmt::LetStmt(ident, expr) => evaluate_let(ident, Box::new(expr), env),
        Stmt::ReturnStmt(expr) => evaluate_return(Box::new(expr), env),
        Stmt::ExprStmt(expr) => evaluate_expression(Box::new(expr), env)
    };

}

pub fn evaluate_let(ident: Ident, expr: Box<Expr>, env: &mut Environment) -> Object {
    let r = evaluate_expression(expr, env);
    env.insert(ident, RefCell::new(r));
    return Object::Null;
}


pub fn evaluate_return(expr: Box<Expr>, env: &mut Environment) -> Object {
    return Object::Ret(Box::new(evaluate_expression(expr, env)));
}

pub fn evaluate_expression(expression: Box<Expr>, env: &mut Environment) -> Object {
    return match *expression {
        Expr::LiteralExpr(literal) => evaluate_literal(literal),
        Expr::Prefix(prefix, expr) => evaluate_prefix(prefix, expr, env),
        Expr::Infix(left, infix, right) => evaluate_infix(left, infix, right, env),
        Expr::If(condition, consequence, alternative) => evaluate_if(condition, consequence, alternative, env),
        Expr::IdentExpr(ident) => evaluate_ident(ident, env),
        Expr::Fn(params, block) => evaluate_closure(params, block, env),
        Expr::Call(ident, args) => evaluate_call(ident, args, env),
        Expr::Array(exprs) => evaluate_array(exprs, env),
        Expr::IndexExpr(arr, index) => evaluate_index(arr, index, env),
        _ => Object::Null
    }
}


fn evaluate_index(arr: Box<Expr>, index: Box<Expr>, env: &mut Environment) -> Object {

    let idx = match evaluate_expression(index, env) {
        Object::Integer(x) => x,
        _ => return Object::Error("Index is not an integer".to_string())
    };


    match evaluate_expression(arr, env){
        Object::Array(a) => evaluate_array_index(a, idx),
        _ => Object::Error("Object is not indexable".to_string())
    }
 }

 fn evaluate_array_index(arr: Vec<Object>, idx: i64) -> Object {
     let max = (arr.len() - 1) as i64;
     if idx < 0 || idx > max {
         return Object::Null;
     } else {
         return arr[idx as usize].clone();
     }
 }


fn evaluate_array(exprs: Vec<Expr>, env: &mut Environment) -> Object {
    return Object::Array(evaluate_expressions(exprs, env));
}



pub fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(x) => x,
        _ => true
    }
}

fn is_error(obj:  &Object) -> bool {
    match obj {
        Object::Error(msg) => true,
        _ => false
    }
}

fn evaluate_builtin(name: String, args: Vec<Object>) -> Object {
    match name.as_str() {
        "len" => {
            if args.len() != 1 {
                return Object::Error("wrong number of arguments".to_string());
            }
            match &args[0] {
                Object::Str(x) => Object::Integer(x.len() as i64),
                Object::Array(x) => Object::Integer(x.len() as i64),
                _ => Object::Error("argument to len not supported".to_string())
            }
        }, 
        "first" => {
            if args.len() != 1 {
                return Object::Error("wrong number of arguments".to_string());
            }
            match &args[0] {
                Object::Array(x) => x[0].clone(),
                _ => Object::Error("argument to first not supported".to_string())
            }
        },
        "last" => {
            if args.len() != 1 {
                return Object::Error("wrong number of arguments".to_string());
            }
            match &args[0] {
                Object::Array(x) => x[x.len()-1].clone(),
                _ => Object::Error("argument to last not supported".to_string())
            }
        },
        "rest" => {
            if args.len() != 1 {
                return Object::Error("wrong number of arguments".to_string());
            }
            match &args[0] {
                Object::Array(x) => Object::Array(x[1..].to_vec()),
                _ => Object::Error("argument to rest not supported".to_string())
            }
        },
        "push" => {
            if args.len() != 2 {
                return Object::Error("wrong number of arguments".to_string());
            }
            match &args[0] {
                Object::Array(x) => {
                    let mut result = x.clone();
                    result.push(args[1].clone());
                    Object::Array(result)
                }
                _ => Object::Error("argument to len not supported".to_string())
            }
        },   
        _ => Object::Error("Ident not found".to_string())
    }
}

pub fn evaluate_call(expr: Box<Expr>, args: Args, env: &mut Environment) -> Object {
    let function = evaluate_expression(expr, env);

    match function {
        Object::Builtin(name) => {
            let args = evaluate_expressions(args, env);
            return evaluate_builtin(name, args)
        },
        Object::Closure(params, block, cenv) => {
            let args = evaluate_expressions(args, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }

            let mut enclosed = cenv.clone();

            for (param_idx, param) in params.iter().enumerate() {
                enclosed.insert(param.clone(), RefCell::new(args[param_idx].clone()));
            }


            return evaluate_block(&block, &mut enclosed);
            

        },
        _ => Object::Error(format!("Not callable: {}", function))
    }
}


pub fn evaluate_expressions(expressions: Vec<Expr>, env: &mut Environment) -> Vec<Object> {
    let mut output = vec![];
    for expression in expressions {
        let exp = evaluate_expression(Box::new(expression), env);
        if let Object::Error(msg) = exp {
            return vec![Object::Error(msg)];
        } else {
            output.push(exp);
        }
    }
    return output;
}

pub fn evaluate_closure(params: Params, block: Block, env: &Environment) -> Object {
    return Object::Closure(params, block, env.clone());
}

pub fn evaluate_if(condition: Box<Expr>, consequence: Block, alternative: Option<Block>, env: &mut Environment) -> Object {
    if is_truthy(evaluate_expression(condition, env)) {
        return evaluate_block(&consequence, env);
    } else if let Some(alt) = alternative {
        return evaluate_block(&alt, env);
    } else {
        return Object::Null
    }

}

pub fn evaluate_infix(left: Box<Expr>, infix: Infix, right: Box<Expr>, env: &mut Environment) -> Object {
    let leftobj = evaluate_expression(left, env);
    let rightobj = evaluate_expression(right, env);
    return match (leftobj, rightobj) {
        (Object::Integer(x), Object::Integer(y)) => evaluate_integer_infix(x, infix, y),
        (Object::Boolean(x), Object::Boolean(y)) => evaluate_boolean_infix(x, infix, y),
        (Object::Str(x), Object::Str(y)) => evaluate_string_infix(x, infix, y),
        _ => Object::Error("type mismatch".to_string())
    }
}

pub fn evaluate_boolean_infix(left: bool, infix: Infix, right: bool) -> Object {
    match infix {
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
        Infix::Plus => Object::Error("unknown operator".to_string()),
        Infix::Minus => Object::Error("unknown operator".to_string()),
        Infix::Multiply => Object::Error("unknown operator".to_string()),
        Infix::Divide => Object::Error("unknown operator".to_string()),
        Infix::LT => Object::Error("unknown operator".to_string()),
        Infix::GT => Object::Error("unknown operator".to_string()),
        Infix::Call => Object::Error("uncallable".to_string()),
        Infix::Index => Object::Error("not indexable".to_string())
    }
}

pub fn evaluate_string_infix(left: String, infix: Infix, right: String) -> Object {
    match infix {
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
        Infix::Plus => Object::Str(format!("{}{}", left, right)),
        Infix::Call => Object::Error("uncallable".to_string()),
        _ => Object::Error("unknown operator".to_string()),
    }
}


pub fn evaluate_integer_infix(left: i64, infix: Infix, right: i64) -> Object {
    match infix {
        Infix::Plus => Object::Integer(left + right),
        Infix::Minus => Object::Integer(left - right),
        Infix::Multiply => Object::Integer(left * right),
        Infix::Divide => Object::Integer(left / right),
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
        Infix::LT => Object::Boolean(left < right),
        Infix::GT => Object::Boolean(left > right),
        _ => Object::Null
    }
}

pub fn evaluate_literal(literal: Literal) -> Object {
    return match literal {
        Literal::Bool(x) => Object::Boolean(x),
        Literal::Int(x) => Object::Integer(x),
        Literal::Str(x) => Object::Str(x)
    }
}

pub fn evaluate_ident(ident: Ident, env: &Environment) -> Object {

    match ident.0.as_str() {
        "len" => Object::Builtin("len".to_string()),
        "first" => Object::Builtin("first".to_string()),
        "last" => Object::Builtin("last".to_string()),
        "rest" => Object::Builtin("rest".to_string()),
        "push" => Object::Builtin("push".to_string()),
        _ => match env.get(&ident) {
            Some(obj) => obj.borrow_mut().clone(),
            None => Object::Error(format!("unknown identifier: {}", ident))
        }
    }
}

fn evaluate_prefix(prefix: Prefix, expr: Box<Expr>, env: &mut Environment) -> Object {

    return match prefix {
        Prefix::Minus => {
            let obj = evaluate_expression(expr, env);
            return match obj {
                Object::Integer(x) => Object::Integer(-x),
                _ => Object::Error("unknown operator".to_string())
            }
        },
        Prefix::Plus => evaluate_expression(expr, env),
        Prefix::Not => {
            let obj = evaluate_expression(expr, env);
            return match obj {
                Object::Boolean(b) => Object::Boolean(!b),
                Object::Null => Object::Boolean(true),
                _ => Object::Boolean(false)
            }
        }
    }
}


pub fn evaluate_block(block: &Block, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmnt in block {
        let r = evaluate_statement(stmnt.clone(), env);
        match r {
            Object::Ret(x) => return Object::Ret(x),
            Object::Error(s) => return Object::Error(s),
            _ => result = r
        }

    }

    return result; 
}

pub fn evaluate(program: Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmnt in program {
        let r = evaluate_statement(stmnt, env);
        match r {
            Object::Ret(x) => return *x,
            Object::Error(s) => return Object::Error(s),
            _ => result = r
        }

    }

    return result; 
}