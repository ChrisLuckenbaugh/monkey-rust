use crate::evaluator::object;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    OpConstant(usize),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpPop,
    OpTrue,
    OpFalse,
}

pub type Instructions = Vec<Instruction>;



