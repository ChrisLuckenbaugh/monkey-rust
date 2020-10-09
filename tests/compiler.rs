extern crate interpreter;
use interpreter::compiler::Compiler;
use interpreter::code::Instruction;
use interpreter::parser::ast::Program;
use interpreter::parser::Parser;
use interpreter::lexer::Lexer;
use interpreter::evaluator::object::Object;

#[test]
fn test_integer_arithmetic() {
    let input = "1+2";
    let expected_constants = vec![Object::Integer(1),Object::Integer(2)];
    let expected_instructions = vec![
        Instruction::OpConstant(0),
        Instruction::OpConstant(1)
    ];

    
    test_run_vm(input, expected_constants, expected_instructions);



}

fn test_run_vm(input: &str, expected_constants: Vec<Object>, expected_instructions: Vec<Instruction>) {
    let program = parse(input);
    let mut compiler = Compiler::new();
    if let Err(x) = compiler.compile(program) {
        panic!("{}", x)
    }

    let bytecode = compiler.bytecode();

    for (i,s) in expected_instructions.iter().enumerate() {
        assert_eq!(*s, bytecode.instructions[i]);
    }

    for (i,s) in expected_constants.iter().enumerate() {
        assert_eq!(*s, *bytecode.constants[i].as_ref());
    }
}


fn parse(input: &str) -> Program {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    match p.parse_program() {
        Ok(program) => program,
        Err(errors) => panic!("Some errors were produced during parsing {:?}", errors)
    }
}
