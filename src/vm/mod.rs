use crate::chunk::Chunk;
use crate::chunk::OperationCode;
use crate::debug::print_value;
use crate::value::Value;

pub struct VirtualMachine;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VirtualMachine {
    pub fn interpret(chunk: Chunk) -> InterpretResult {
        VirtualMachine::run(chunk)
    }

    fn run(chunk: Chunk) -> InterpretResult {
        let mut stack: Vec<Value> = vec![];

        for op_code in chunk.codes {

            // NOTE: This is for debugging
            println!("\t{:?}", stack);

            match op_code {
                OperationCode::Return => {
                    print_value(stack.pop().unwrap());
                    // TODO: CHECK unwrap.
                    return InterpretResult::Ok;
                }
                OperationCode::Constant(constant) => stack.push(constant),
                OperationCode::Add
                | OperationCode::Substract
                | OperationCode::Multiply
                | OperationCode::Divide => {
                    VirtualMachine::binary_operation(op_code, &mut stack);
                }
                OperationCode::Negate => {
                    let negation = -stack.pop().unwrap();
                    stack.push(negation);
                }
            };
        }

        InterpretResult::Ok
    }

    fn binary_operation(op_code: OperationCode, stack: &mut Vec<Value>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match op_code {
            OperationCode::Add => stack.push(a + b),
            OperationCode::Substract => stack.push(a - b),
            OperationCode::Multiply => stack.push(a * b),
            OperationCode::Divide => stack.push(a / b),
            _ => unreachable!(),
        }
    }
}
