use crate::{code, parser::ast::*};
use crate::evaluator::object::Object;
use std::error::Error;
use std::{rc::Rc};

pub struct  Compiler {
    instructions: code::Instructions,
    constants: Vec<Rc<Object>>
}


pub struct  ByteCode {
    pub instructions: code::Instructions,
    pub constants: Vec<Rc<Object>>
}


impl Compiler {
    pub fn new() -> Self {
        return Compiler{
            instructions: vec![],
            constants: vec![]
        }
    }

    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(Rc::new(obj));
        return self.constants.len() - 1;
    }

    pub fn compile_literal(&mut self, lit: Literal) -> Result<(), Box<Error>> {
        match lit {
            Literal::Int(x) => {
                let id = self.add_constant(Object::Integer(x));
                self.instructions.push(code::Instruction::OpConstant(id));
            },
            Literal::Bool(x) => {
                let op = if x { code::Instruction::OpTrue } else { code::Instruction::OpFalse };
                self.instructions.push(op)
            },
            _ => return Err("Not supported yet!".into())
        }
        Ok(())
    }

    fn push_infix(&mut self, op: &Infix) -> Result<(), Box<Error>> {
        let ins = match op {
            Infix::Plus => code::Instruction::OpAdd,
            Infix::Multiply => code::Instruction::OpMultiply,
            Infix::Minus => code::Instruction::OpSubtract,
            Infix::Divide => code::Instruction::OpDivide,
            _ => return Err("Not supported yet!".into())
        };

        self.instructions.push(ins);
        Ok(())
    }

    pub fn compile_expr(&mut self, expr: Expr) -> Result<(), Box<Error>> {
        match expr {
            Expr::Infix(left,op,right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.push_infix(&op)?;
            },
            Expr::LiteralExpr(lit) => self.compile_literal(lit)?,
            _ => return Err("Not supported yet!".into())
        }
        return Ok(());
    }

    pub fn compile_statement(&mut self, stmt: Stmt) -> Result<(), Box<Error>> {
        match stmt {
            Stmt::ExprStmt(x) => {
                self.compile_expr(x)?;
                self.instructions.push(code::Instruction::OpPop);
                Ok(())
            },
            _ => Err("Not supported yet!".into())
        }
    }

    pub fn compile(&mut self, program: Program) -> Result<(), Box<Error>> {
        for stmnt in program {
            self.compile_statement(stmnt)?;
        }

        return Ok(())
    }


    pub fn bytecode(&self) -> ByteCode {
        return ByteCode{ instructions: self.instructions.clone(), constants: self.constants.clone() }
    }



}