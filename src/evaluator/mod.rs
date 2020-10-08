pub mod builtins;
pub mod object;
use crate::parser::ast::*;
use std::fmt;
use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use object::Object;



pub type Environment = std::collections::HashMap<Ident,Rc<RefCell<Object>>>;

pub fn evaluate_statement(statement: &Stmt, env: &mut Environment) -> Object {
    return match statement {
        Stmt::LetStmt(ident, expr) => evaluate_let(&ident, expr, env),
        Stmt::ReturnStmt(expr) => evaluate_return(expr, env),
        Stmt::ExprStmt(expr) => evaluate_expression(expr, env)
    };

}

pub fn evaluate_let(ident: &Ident, expr: &Expr, env: &mut Environment) -> Object {
    let r = evaluate_expression(expr, env);
    env.insert(ident.clone(), Rc::new(RefCell::new(r)));
    return Object::Null;
}


pub fn evaluate_return(expr: &Expr, env: &mut Environment) -> Object {
    return Object::Ret(Box::new(evaluate_expression(expr, env)));
}

pub fn evaluate_expression(expression: &Expr, env: &mut Environment) -> Object {
    return match expression {
        Expr::LiteralExpr(literal) => evaluate_literal(literal, env),
        Expr::Prefix(prefix, expr) => evaluate_prefix(prefix, expr, env),
        Expr::Infix(left, infix, right) => evaluate_infix(left, infix, right, env),
        Expr::If(condition, consequence, alternative) => evaluate_if(condition, consequence, alternative, env),
        Expr::IdentExpr(ident) => evaluate_ident(ident, env),
        Expr::Fn(params, block) => evaluate_closure(&params, &block, env),
        Expr::Call(ident, args) => evaluate_call(ident, args, env),
        Expr::Array(exprs) => evaluate_array(exprs, env),
        Expr::IndexExpr(arr, index) => evaluate_index(arr, index, env),
        _ => Object::Null
    }
}


fn evaluate_index(arr: &Expr, index: &Expr, env: &mut Environment) -> Object {

    match evaluate_expression(&arr, env){
        Object::Array(a) => {
            let idx = match evaluate_expression(&index, env) {
                Object::Integer(x) => x,
                _ => return Object::Error("Index is not an integer".to_string())
            };
            evaluate_array_index(a.as_ref(), idx)
        }
        Object::Hash(h) => {
            let idx = evaluate_expression(index, env);

            return match h.get(&idx) {
                Some(o) => o.clone(),
                None => Object::Error("Not found in map".to_string())
            }
        }
        _ => Object::Error("Object is not indexable".to_string())
    }
 }

 fn evaluate_array_index(arr: &Vec<Object>, idx: i64) -> Object {
     let max = (arr.len() - 1) as i64;
     if idx < 0 || idx > max {
         return Object::Null;
     } else {
         return arr[idx as usize].clone();
     }
 }


fn evaluate_array(exprs: &Vec<Expr>, env: &mut Environment) -> Object {
    return Object::Array(Rc::new(evaluate_expressions(exprs, env)));
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
pub fn evaluate_call(expr: &Expr, args: &Args, env: &mut Environment) -> Object {
    let function = evaluate_expression(&expr, env);

    match function {
        Object::Builtin(name) => {
            let args = evaluate_expressions(args, env);
            return name(args)
        },
        Object::Closure(params, block, cenv) => {
            let args = evaluate_expressions(args, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }

            let mut enclosed = cenv.clone();

            for (param_idx, param) in params.iter().enumerate() {
                enclosed.insert(param.clone(), Rc::new(RefCell::new(args[param_idx].clone())));
            }


            return evaluate_block(&block, &mut enclosed);
            

        },
        _ => Object::Error(format!("Not callable: {}", function))
    }
}


pub fn evaluate_expressions(expressions: &Vec<Expr>, env: &mut Environment) -> Vec<Object> {
    let mut output = vec![];
    for expression in expressions {
        let exp = evaluate_expression(&expression, env);
        if let Object::Error(msg) = exp {
            return vec![Object::Error(msg)];
        } else {
            output.push(exp);
        }
    }
    return output;
}

pub fn evaluate_closure(params: &Rc<Params>, block: &Rc<Block>, env: &Environment) -> Object {
    return Object::Closure(Rc::clone(params), Rc::clone(block), env.clone());
}

pub fn evaluate_if(condition: &Expr, consequence: &Block, alternative: &Option<Block>, env: &mut Environment) -> Object {
    if is_truthy(evaluate_expression(&condition, env)) {
        return evaluate_block(&consequence, env);
    } else if let Some(alt) = alternative {
        return evaluate_block(&alt, env);
    } else {
        return Object::Null
    }

}

pub fn evaluate_infix(left: &Expr, infix: &Infix, right: &Expr, env: &mut Environment) -> Object {
    let leftobj = evaluate_expression(&left, env);
    let rightobj = evaluate_expression(&right, env);
    return match (leftobj, rightobj) {
        (Object::Integer(x), Object::Integer(y)) => evaluate_integer_infix(x, infix, y),
        (Object::Boolean(x), Object::Boolean(y)) => evaluate_boolean_infix(x, infix, y),
        (Object::Str(x), Object::Str(y)) => evaluate_string_infix(x.as_ref(), infix, y.as_ref()),
        _ => Object::Error("type mismatch".to_string())
    }
}

pub fn evaluate_boolean_infix(left: bool, infix: &Infix, right: bool) -> Object {
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

pub fn evaluate_string_infix(left: &String, infix: &Infix, right: &String) -> Object {
    match infix {
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
        Infix::Plus => Object::Str(Rc::new(format!("{}{}", left, right))),
        Infix::Call => Object::Error("uncallable".to_string()),
        _ => Object::Error("unknown operator".to_string()),
    }
}


pub fn evaluate_integer_infix(left: i64, infix: &Infix, right: i64) -> Object {
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

pub fn evaluate_literal(literal: &Literal, env: &mut Environment) -> Object {
    return match literal {
        Literal::Bool(x) => Object::Boolean(*x),
        Literal::Int(x) => Object::Integer(*x),
        Literal::Str(x) => Object::Str(Rc::clone(x)),
        Literal::Hash(x) => {
            let mut m: HashMap<Object,Object> = HashMap::new();

            for (key,value) in x {
                let key_obj = match evaluate_expression(&key, env) {
                    Object::Integer(x) => Object::Integer(x),
                    Object::Boolean(x) => Object::Boolean(x),
                    Object::Str(x) => Object::Str(x),
                    _ => return Object::Error("invalid key object for hash".to_string())
                };

                let val_obj = evaluate_expression(&value, env);
                if let Object::Error(x) = val_obj {
                    return Object::Error(x);
                }

                m.insert(key_obj,val_obj);

            }
            return Object::Hash(m);
        }
    }
} 


pub fn evaluate_ident(ident: &Ident, env: &Environment) -> Object {

    match ident.0.as_str() {
        "len" => Object::Builtin(builtins::len),
        "first" => Object::Builtin(builtins::first),
        "last" => Object::Builtin(builtins::last),
        "rest" => Object::Builtin(builtins::rest),
        "push" => Object::Builtin(builtins::push),
        "puts" => Object::Builtin(builtins::puts),
        _ => match env.get(&ident) {
            Some(obj) => obj.borrow_mut().clone(),
            None => Object::Error(format!("unknown identifier: {}", ident))
        }
    }
}

fn evaluate_prefix(prefix: &Prefix, expr: &Expr, env: &mut Environment) -> Object {

    return match prefix {
        Prefix::Minus => {
            let obj = evaluate_expression(&expr, env);
            return match obj {
                Object::Integer(x) => Object::Integer(-x),
                _ => Object::Error("unknown operator".to_string())
            }
        },
        Prefix::Plus => evaluate_expression(&expr, env),
        Prefix::Not => {
            let obj = evaluate_expression(&expr, env);
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
        let r = evaluate_statement(stmnt, env);
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
        let r = evaluate_statement(&stmnt, env);
        match r {
            Object::Ret(x) => return *x,
            Object::Error(s) => return Object::Error(s),
            _ => result = r
        }

    }

    return result; 
}