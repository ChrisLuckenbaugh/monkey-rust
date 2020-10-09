extern crate interpreter;

use std::io::{self, BufRead, Write};
use std::error::Error;

use interpreter::lexer::Lexer;
use interpreter::parser::Parser;
use interpreter::evaluator;
use interpreter::compiler::Compiler;
use interpreter::vm::VM;



fn print_errors(errors: &Vec<Box<Error>>) {
    let monkey_face = "             __,__
    .--.  .-\"     \"-.  .--.
   / .. \\/  .-. .-.  \\/ .. \\
  | |  '|  /   Y   \\  |'  | |
  | \\   \\  \\ 0 | 0 /  /   / |
   \\ '- ,\\.-\"\"\"\"\"\"\"-./, -' /
    ''-' /_   ^ ^   _\\ '-''
        |  \\._   _./  |
        \\   \\ '~' /   /
         '._ '-=-' _.'
            '-----'
 ";
    print!("{}", monkey_face);
    print!("Whoops we ran into some monkey business here!");
    println!(" parser errors:");
    for error in errors.iter() {
        println!("\t{}", error);
    }
}

fn start() {
    let prompt = ">> ";
    let mut env = evaluator::Environment::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);

        let mut l = Lexer::new(&buffer.as_str());
        let mut p = Parser::new(&mut l);

        match p.parse_program() {
            Ok(program) => {
                let mut compiler = Compiler::new();
                if let Err(x) = compiler.compile(program) {
                    println!("{}", x)
                }

                let mut machine = VM::new(compiler.bytecode());
                if let Err(x) = machine.run() {
                    println!("{}", x)
                }

                match machine.last_popped() {
                    Some(x) => println!("{}", x),
                    None => {}
                }

                
                /*
                let evaluated = evaluator::evaluate(program, &mut env);
                match evaluated {
                    evaluator::object::Object::Null => continue,
                    _ => println!("{}", evaluated)
                }*/
            },
            Err(errors) => print_errors(errors)
        }

    }
}

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    start();
}