use crate::chunk::op_code::OperationCode::{self, *};
use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::value::Value;

pub struct VirtualMachine;

#[derive(PartialEq, Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VirtualMachine {
    pub fn interpret(source: &str) -> InterpretResult {
        let mut chunk = Chunk::create();
        if compile(&mut chunk, source) {
            println!("{:?}", chunk);
            VirtualMachine::run(chunk)
        } else {
            InterpretResult::CompileError
        }

        // compile(source);
        // InterpretResult::Ok
    }

    fn run(chunk: Chunk) -> InterpretResult {
        let mut stack: Vec<Value> = vec![];

        for (code, _) in chunk.codes.iter() {
            match code {
                Return => {
                    println!("{}", stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                Constant(constant) => stack.push(*constant),
                Add | Substract | Multiply | Divide => VirtualMachine::binary_operation(code, &mut stack),
                Negate => {
                    let negation = -stack.pop().unwrap();
                    stack.push(negation);
                }
            };
        }

        InterpretResult::Ok
    }

    fn binary_operation(code: &OperationCode, stack: &mut Vec<Value>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match code {
            Add => stack.push(a + b),
            Substract => stack.push(a - b),
            Multiply => stack.push(a * b),
            Divide => stack.push(a / b),
            _ => unreachable!(),
        }
    }
}
