extern crate interpreter;
use interpreter::compiler::Compiler;
use interpreter::code::Instruction;
use interpreter::parser::ast::Program;
use interpreter::parser::Parser;
use interpreter::lexer::Lexer;
use interpreter::evaluator::object::Object;
use interpreter::vm::VM;

#[test]
fn test_integer_arithmetic() {


    test_run_vm("1+2", &Object::Integer(3));
    test_run_vm("1-2", &Object::Integer(-1));
    test_run_vm("1*2", &Object::Integer(2));
    test_run_vm("4/2", &Object::Integer(2));
    test_run_vm("50 / 2 * 2 + 10 - 5", &Object::Integer(55));
    test_run_vm("5 + 5 + 5 + 5 - 10", &Object::Integer(10));
    test_run_vm("2 * 2 * 2 * 2 * 2", &Object::Integer(32));
    test_run_vm("5*2+10", &Object::Integer(20));
    test_run_vm("5+2*10", &Object::Integer(25));
    test_run_vm("5*(2+10)", &Object::Integer(60));
}

#[test]
fn test_boolean() {


    test_run_vm("true", &Object::Boolean(true));
    test_run_vm("false", &Object::Boolean(false));
}


fn test_run_vm(input: &str, expected: &Object) {
    let program = parse(input);
    let mut compiler = Compiler::new();
    if let Err(x) = compiler.compile(program) {
        panic!("{}", x)
    }

    let mut vm = VM::new(compiler.bytecode());
    if let Err(x) = vm.run() {
        panic!("{}", x)
    }

    assert_eq!(vm.last_popped(), Some(expected));
}


fn parse(input: &str) -> Program {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    match p.parse_program() {
        Ok(program) => program,
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }
}
