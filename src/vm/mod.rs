
use crate::evaluator::object::Object;
use crate::{code::*, compiler};
use std::error::Error;
use std::{rc::Rc};

pub struct VM {
    constants: Vec<Rc<Object>>,
    instructions: Instructions,
    stack: Vec<Rc<Object>>,
    last_popped: Option<Rc<Object>>
}

impl VM {
    pub fn new(bytecode: compiler::ByteCode ) -> Self {
        return VM{
            instructions: bytecode.instructions,
            constants: bytecode.constants,
            stack: vec![],
            last_popped: None
        }
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::OpConstant(x) => self.stack.push(Rc::clone(&self.constants[*x])),
                Instruction::OpTrue => self.stack.push(Rc::new(Object::Boolean(true))),
                Instruction::OpFalse => self.stack.push(Rc::new(Object::Boolean(true))),
                Instruction::OpPop => {
                    self.last_popped = self.stack.pop();
                },
                Instruction::OpAdd => {
                    let a = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let b = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let res = match (a.as_ref(),b.as_ref()) {
                        (Object::Integer(x),Object::Integer(y)) => Object::Integer(x+y),
                        _ => return Err("Add not supported for these operands".into())
                    };
            
            
                    self.stack.push(Rc::new(res));
                },
                Instruction::OpSubtract => {
                    let a = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let b = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let res = match (a.as_ref(),b.as_ref()) {
                        (Object::Integer(x),Object::Integer(y)) => Object::Integer(y-x),
                        _ => return Err("Add not supported for these operands".into())
                    };
            
            
                    self.stack.push(Rc::new(res));
                },
                Instruction::OpMultiply => {
                    let a = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let b = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let res = match (a.as_ref(),b.as_ref()) {
                        (Object::Integer(x),Object::Integer(y)) => Object::Integer(x*y),
                        _ => return Err("Add not supported for these operands".into())
                    };
            
            
                    self.stack.push(Rc::new(res));
                },
                Instruction::OpDivide => {
                    let a = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let b = match self.stack.pop() {
                        Some(x) => x,
                        None => return Err("Not enough operands on stack".into())
                    };
            
                    let res = match (a.as_ref(),b.as_ref()) {
                        (Object::Integer(x),Object::Integer(y)) => Object::Integer(y/x),
                        _ => return Err("Add not supported for these operands".into())
                    };
            
            
                    self.stack.push(Rc::new(res));
                }

            }
        }
        Ok(())
    }


    pub fn stack_top(&self) -> Option<&Object> {
        let rc = self.stack.last()?;
        return Some(rc.as_ref());
    }


    
    pub fn last_popped(&self) -> Option<&Object> {
        let rc = self.last_popped.as_ref()?;
        return Some(rc.as_ref());
    }

}

