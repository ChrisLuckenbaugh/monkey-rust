extern crate interpreter;
use interpreter::lexer::token::{Token};
use interpreter::lexer::{Lexer};
use interpreter::parser::{Parser};
use interpreter::parser::ast::*;



#[test]
fn idx_expr() {
    let input = "myArray[1 + 1]";


    let expected = vec![
        Stmt::ExprStmt(
            Expr::IndexExpr(
                Box::new(Expr::IdentExpr(Ident("myArray".to_string()))), 
                Box::new(
                    Expr::Infix(
                        Box::new(Expr::LiteralExpr(Literal::Int(1))),
                        Infix::Plus,
                        Box::new(Expr::LiteralExpr(Literal::Int(1))),
                    )
                )
            )
        )
    ];


    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}
#[test]
fn array_expr() {
    let input = "[1, 2 * 2, 3 + 3];";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let l: Vec<Expr> = vec![
        Expr::LiteralExpr(Literal::Int(1)),
        Expr::Infix(
            Box::new(Expr::LiteralExpr(Literal::Int(2))),
            Infix::Multiply,
            Box::new(Expr::LiteralExpr(Literal::Int(2))),
        ),
        Expr::Infix(
            Box::new(Expr::LiteralExpr(Literal::Int(3))),
            Infix::Plus,
            Box::new(Expr::LiteralExpr(Literal::Int(3))),
        )
    ];
    let expected = vec![
        Stmt::ExprStmt(Expr::Array(l))
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }
}

#[test]
fn let_statement() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::LetStmt(Ident("x".to_string()), Expr::LiteralExpr(Literal::Int(5))),
        Stmt::LetStmt(Ident("y".to_string()), Expr::LiteralExpr(Literal::Int(10))),
        Stmt::LetStmt(Ident("foobar".to_string()), Expr::LiteralExpr(Literal::Int(838383))),

    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn return_statement() {
    let input = "
    return 5;
    return 10;
    return 993322;
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ReturnStmt(Expr::LiteralExpr(Literal::Int(5))),
        Stmt::ReturnStmt(Expr::LiteralExpr(Literal::Int(10))),
        Stmt::ReturnStmt(Expr::LiteralExpr(Literal::Int(993322))),
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}


#[test]
fn ident_expression() {
    let input = "
        foobar;
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(Expr::IdentExpr(Ident("foobar".to_string())))
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}


#[test]
fn literal_expression() {
    let input = "
        5;
        true;
        false;
        \"hello world\";
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(Expr::LiteralExpr(Literal::Int(5))),
        Stmt::ExprStmt(Expr::LiteralExpr(Literal::Bool(true))),
        Stmt::ExprStmt(Expr::LiteralExpr(Literal::Bool(false))),
        Stmt::ExprStmt(Expr::LiteralExpr(Literal::Str("hello world".to_string())))
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn prefix_expr() {
    let input = "
        !5;
        -15;
        +7
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(Expr::Prefix(Prefix::Not, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Prefix(Prefix::Minus, Box::new(Expr::LiteralExpr(Literal::Int(15))))),
        Stmt::ExprStmt(Expr::Prefix(Prefix::Plus, Box::new(Expr::LiteralExpr(Literal::Int(7)))))
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn infix_expr() {
    let input = "
        5+5;
        5-5;
        5*5;
        5/5;
        5>5;
        5<5;
        5==5;
        5 != 5;
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::Plus, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::Minus, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::Multiply, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::Divide, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::GT, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::LT, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::Equal, Box::new(Expr::LiteralExpr(Literal::Int(5))))),
        Stmt::ExprStmt(Expr::Infix(Box::new(Expr::LiteralExpr(Literal::Int(5))), Infix::NotEqual, Box::new(Expr::LiteralExpr(Literal::Int(5)))))
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn grouped_expr() {
    let input = "
        2 / (5 + 5);
        -(5 + 5)
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(
            Expr::Infix(
                Box::new(Expr::LiteralExpr(Literal::Int(2))),
                Infix::Divide,
                Box::new(Expr::Infix(
                    Box::new(Expr::LiteralExpr(Literal::Int(5))),
                    Infix::Plus,
                    Box::new(Expr::LiteralExpr(Literal::Int(5)))
                )),
                ),
        ),
        Stmt::ExprStmt(
            Expr::Prefix(
                Prefix::Minus,
                Box::new(Expr::Infix(
                    Box::new(Expr::LiteralExpr(Literal::Int(5))),
                    Infix::Plus,
                    Box::new(Expr::LiteralExpr(Literal::Int(5)))
                ))
            )
        )

    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn if_expr() {
    let input = "
        if (x < y) { x };
        if (x < y) { x } else { y };
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(
            Expr::If(
                Box::new(Expr::Infix(Box::new(Expr::IdentExpr(Ident("x".to_string()))), Infix::LT, Box::new(Expr::IdentExpr(Ident("y".to_string()))))),
                vec![Stmt::ExprStmt(Expr::IdentExpr(Ident("x".to_string())))],
                None
            ),
            
        ),
        Stmt::ExprStmt(
            Expr::If(
                Box::new(Expr::Infix(Box::new(Expr::IdentExpr(Ident("x".to_string()))), Infix::LT, Box::new(Expr::IdentExpr(Ident("y".to_string()))))),
                vec![Stmt::ExprStmt(Expr::IdentExpr(Ident("x".to_string())))],
                Some(vec![Stmt::ExprStmt(Expr::IdentExpr(Ident("y".to_string())))]),
            ),
            
        )

    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn fn_expr() {
    let input = "
        fn (x, y) { x + y; }
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(
            Expr::Fn(
                vec![
                    Ident("x".to_string()),
                    Ident("y".to_string())
                ],
                vec![
                    Stmt::ExprStmt(Expr::Infix(Box::new(Expr::IdentExpr(Ident("x".to_string()))), Infix::Plus, Box::new(Expr::IdentExpr(Ident("y".to_string())))))
                ],
            )
        ),
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}

#[test]
fn call_expr() {
    let input = "
        add( a, b);
    ";

    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);


    let expected = vec![
        Stmt::ExprStmt(
            Expr::Call(
                Box::new(Expr::IdentExpr(Ident("add".to_string()))),
                vec![
                    Expr::IdentExpr(Ident("a".to_string())),
                    Expr::IdentExpr(Ident("b".to_string()))
                ]
            )
        ),
    ];

    match p.parse_program() {
        Ok(program) => {
            for (i,s) in expected.iter().enumerate() {
                assert_eq!(*s, program[i]);
            }
        },
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }

}