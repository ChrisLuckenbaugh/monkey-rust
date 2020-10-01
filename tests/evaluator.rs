extern crate interpreter;
use interpreter::lexer::{Lexer};
use interpreter::parser::{Parser};
use interpreter::parser::ast::*;
use interpreter::evaluator::{Object,evaluate, Environment};
#[test]
fn test_eval_integer() {

    let test_case = vec![
        "5",
        "10",
        "-5",
        "-10",
        "5 + 5 + 5 + 5 - 10",
        "2 * 2 * 2 * 2 * 2",
        "-50 + 100 + -50",
        "5 * 2 + 10",
        "5 + 2 * 10",
        "20 + 2 * -10",
        "50 / 2 * 2 + 10",
        "2 * (5 + 10)",
        "3 * 3 * 3 + 10",
        "3 * (3 * 3) + 10",
        "(5 + 10 * 2 + 15 / 3) * 2 + -10"
    ];

    let expected = vec![
        Object::Integer(5),
        Object::Integer(10),
        Object::Integer(-5),
        Object::Integer(-10),
        Object::Integer(10),
        Object::Integer(32),
        Object::Integer(0),
        Object::Integer(20),
        Object::Integer(25),
        Object::Integer(0),
        Object::Integer(60),
        Object::Integer(30),
        Object::Integer(37),
        Object::Integer(37),
        Object::Integer(50),
    ];
    

    for (i,s) in expected.iter().enumerate() {
        let result = test_eval(test_case[i].to_string());
        println!("{:?}", result);
        assert_eq!(*s, result);
    }
}


#[test]
fn test_eval_str() {
    let test_case = vec![
        "\"Hello World\"",
        "\"Hello\" + \" \" + \"World\""
    ];

    let expected = vec![
        Object::Str("Hello World".to_string()),
        Object::Str("Hello World".to_string())
    ];

    for (i,s) in expected.iter().enumerate() {
        let result = test_eval(test_case[i].to_string());
        println!("{:?}", result);
        assert_eq!(*s, result);
    }
}

#[test]
fn test_eval_bool() {

    let test_case = vec![
        "true",
        "false",
        "1 < 2",
        "1 > 2",
        "1 < 1",
        "1 > 1",
        "1 == 1",
        "1 != 1",
        "1 == 2",
        "1 != 2",
        "true == true",
        "false == false",
        "true == false",
        "true != false",
        "false != true",
        "(1 < 2) == true",
        "(1 < 2) == false",
        "(1 > 2) == true",
        "(1 > 2) == false"
    ];

    let expected = vec![
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(false),
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(false),
        Object::Boolean(true),
    ];

    for (i,s) in expected.iter().enumerate() {
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

fn test_eval(input: String) -> Object {
    let mut l = Lexer::new(&input.as_str());
    let mut p = Parser::new(&mut l);


    match p.parse_program() {
        
        Ok(program) => {
            let mut env = Environment::new();
            return evaluate(program, &mut env)
        },
        Err(error) => panic!("Error during parsing {:?}", error)
    }
}

#[test]
fn test_eval_not() {

    let test_case = vec![
        "!true",
        "!false",
        "!!true",
        "!!false",
        "!!5"
    ];

    let expected = vec![
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Boolean(false),
        Object::Boolean(true)
    ];

    for (i,s) in expected.iter().enumerate() {
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

#[test]
fn test_conditional() {

    let test_case = vec![
        "if (true) { 10 }",
        "if (false) { 10 }",
        "if (1) { 10 }",
        "if (1 < 2) { 10 }",
        "if (1 > 2) { 10 }",
        "if (1 > 2) { 10 } else  { 20 }",
        "if (1 < 2) { 10 } else { 20 }"
    ];

    let expected = vec![
        Object::Integer(10),
        Object::Null,
        Object::Integer(10),
        Object::Integer(10),
        Object::Null,
        Object::Integer(20),
        Object::Integer(10)
    ];

    for (i,s) in expected.iter().enumerate() {
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

#[test]
fn test_return() {

    let test_case = vec![
        "return 10;",
        "return 10; 9;",
        "return 2 * 5; 9;",
        "9; return 2 * 5; 9;",
        "
        if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }            
        "
    ];

    let expected = vec![
        Object::Integer(10),
        Object::Integer(10),
        Object::Integer(10),
        Object::Integer(10),
        Object::Integer(10),
    ];

    for (i,s) in expected.iter().enumerate() {
        println!("{}", test_eval(test_case[i].to_string()));
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

#[test]
fn test_error() {

    let test_case = vec![         
        "5 + true;",
        "5 + true; 5;",
        "-true",
        "true + false",
        "5; true + false; 5",
        "if (10 > 1) { true + false }",
        "\"Hello\" - \"World\""
    ];

    let expected = vec![
        Object::Error("type mismatch".to_string()),
        Object::Error("type mismatch".to_string()),
        Object::Error("unknown operator".to_string()),
        Object::Error("unknown operator".to_string()),
        Object::Error("unknown operator".to_string()),
        Object::Error("unknown operator".to_string()),
        Object::Error("unknown operator".to_string()),
    ];

    for (i,s) in expected.iter().enumerate() {
        println!("{}", test_eval(test_case[i].to_string()));
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}


#[test]
fn test_let_statement() {

    let test_case = vec![         
        "let a = 5; a;",
        "let a = 5 * 5; a;",
        "let a = 5; let b = a; b;",
        "let a = 5; let b = a; let c = a + b + 5; c;"
    ];

    let expected = vec![
        Object::Integer(5),
        Object::Integer(25),
        Object::Integer(5),
        Object::Integer(15)
    ];

    for (i,s) in expected.iter().enumerate() {
        println!("{}", test_eval(test_case[i].to_string()));
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

#[test]
fn test_function_object() {

    let test_case = vec![         
        "fn(x) { x + 2; };"
    ];

    let expected = vec![
        Object::Closure(
            vec![
                Ident("x".to_string()),
            ],
            vec![
                Stmt::ExprStmt(Expr::Infix(
                    Box::new(Expr::IdentExpr(Ident("x".to_string()))),
                    Infix::Plus,
                    Box::new(Expr::LiteralExpr(Literal::Int(2)))
                ))
            ],
            Environment::new()
        )
    ];

    for (i,s) in expected.iter().enumerate() {
        println!("{}", test_eval(test_case[i].to_string()));
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

#[test]
fn test_function_application() {

    let test_case = vec![         
        "let identity = fn(x) { x; }; identity(5);",
        "let identity = fn(x) { return x; }; identity(5);",
        "let double = fn(x) { x * 2; }; double(5);",
        "let add = fn(x, y) { x + y; }; add(5, 5);",
        "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
        "fn(x) { x; } (5)",
    ];

    let expected = vec![
        Object::Integer(5),
        Object::Integer(5),
        Object::Integer(10),
        Object::Integer(10),
        Object::Integer(20),
        Object::Integer(5),
    ];

    for (i,s) in expected.iter().enumerate() {
        println!("{}", test_eval(test_case[i].to_string()));
        assert_eq!(*s, test_eval(test_case[i].to_string()));
    }
}

