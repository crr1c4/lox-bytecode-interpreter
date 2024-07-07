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

        for (code, line) in chunk.codes.iter() {
            match code {
                Return => {
                    println!("{}", stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                Constant(constant) => stack.push(*constant),
                Add | Substract | Multiply | Divide => match VirtualMachine::binary_operation(code, *line, &mut stack) {
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    _ => (),
                },
                Negate => match stack.pop() {
                    Some(Value::Number(number)) => {
                        let negation = -number;
                        stack.push(Value::Number(negation))
                    }
                    _ => return VirtualMachine::throw_runtime_error("Operand must be a number", *line),
                },
            };
        }

        InterpretResult::Ok
    }

    fn binary_operation(code: &OperationCode, line: u32, stack: &mut Vec<Value>) -> InterpretResult {
        let b = match stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return VirtualMachine::throw_runtime_error("Operand must be a number", line),
        };

        let a = match stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return VirtualMachine::throw_runtime_error("Operand must be a number", line),
        };

        let result = match code {
            Add => a + b,
            Substract => a - b,
            Multiply => a * b,
            Divide => a / b,
            _ => unreachable!(),
        };

        stack.push(Value::Number(result));

        InterpretResult::Ok
    }

    // TODO: Check args
    fn throw_runtime_error(message: &str, line: u32) -> InterpretResult {
        eprintln!("{} [line {}] in script.", message, line);
        InterpretResult::RuntimeError
    }
}
