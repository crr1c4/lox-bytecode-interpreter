use crate::chunk::op_code::OperationCode::{self, *};
use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::value::object::Object;
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
                Constant(constant) => stack.push(constant.clone()),
                Add => match (stack.pop(), stack.pop()) {
                    (Some(Value::Number(b)), Some(Value::Number(a))) => stack.push(Value::Number(a + b)),
                    (Some(Value::Object(b)), Some(Value::Object(a))) => match VirtualMachine::concatenate(a, b, &mut stack) {
                        InterpretResult::RuntimeError => return VirtualMachine::throw_runtime_error("Operands must be a number or strings", *line),
                        _ => (),
                    },
                    (_, _) => (),
                },

                Substract | Multiply | Divide => match VirtualMachine::binary_operation(&mut stack, code) {
                    InterpretResult::RuntimeError => return VirtualMachine::throw_runtime_error("Operands must be a number", *line),
                    _ => (),
                },
                Negate => match stack.pop() {
                    Some(Value::Number(number)) => {
                        let negation = -number;
                        stack.push(Value::Number(negation))
                    }
                    _ => return VirtualMachine::throw_runtime_error("Operand must be a number", *line),
                },
                Nil => stack.push(Value::Nil),
                True => stack.push(Value::Bool(true)),
                False => stack.push(Value::Bool(false)),
                Not => {
                    let value = stack.pop().unwrap();
                    let value = VirtualMachine::is_falsey(value);
                    stack.push(Value::Bool(value));
                }
                Equal => match VirtualMachine::values_equal(&mut stack) {
                    InterpretResult::RuntimeError => return VirtualMachine::throw_runtime_error("Operand must be a number", *line),
                    _ => (),
                },
                Greater | Less => match VirtualMachine::binary_boolean_operation(code, &mut stack) {
                    InterpretResult::RuntimeError => return VirtualMachine::throw_runtime_error("Operand must be a number", *line),
                    _ => (),
                },
            };
        }

        InterpretResult::Ok
    }

    fn binary_operation(stack: &mut Vec<Value>, code: &OperationCode) -> InterpretResult {
        let (b, a) = match (stack.pop(), stack.pop()) {
            (Some(Value::Number(b)), Some(Value::Number(a))) => (b, a),
            _ => return InterpretResult::RuntimeError,
        };

        let result = match code {
            // Add => a + b,
            Substract => a - b,
            Multiply => a * b,
            Divide => a / b,
            _ => unreachable!(),
        };

        stack.push(Value::Number(result));

        InterpretResult::Ok
    }

    fn concatenate(a: Box<dyn Object>, b: Box<dyn Object>, stack: &mut Vec<Value>) -> InterpretResult {
        let mut a = a.to_string();
        let b = b.to_string();

        a.push_str(&b);

        stack.push(Value::Object(Box::new(a)));

        InterpretResult::Ok
    }

    fn binary_boolean_operation(code: &OperationCode, stack: &mut Vec<Value>) -> InterpretResult {
        let b = match stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return InterpretResult::RuntimeError,
        };

        let a = match stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return InterpretResult::RuntimeError,
        };

        let result = match code {
            Less => a < b,
            Greater => a > b,
            _ => unreachable!(),
        };

        stack.push(Value::Bool(result));

        InterpretResult::Ok
    }

    fn throw_runtime_error(message: &str, line: u32) -> InterpretResult {
        eprintln!("{} [line {}] in script.", message, line);
        InterpretResult::RuntimeError
    }

    fn is_falsey(value: Value) -> bool {
        match value {
            Value::Nil | Value::Bool(_) => true,
            Value::Number(_) => false,
            _ => unreachable!(),
        }
    }

    fn values_equal(stack: &mut Vec<Value>) -> InterpretResult {
        let b = match stack.pop() {
            Some(value) => value,
            _ => return InterpretResult::RuntimeError,
        };

        let a = match stack.pop() {
            Some(value) => value,
            _ => return InterpretResult::RuntimeError,
        };

        let result = match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Object(x), Value::Object(y)) => {
                let x = x.to_string();
                let y = y.to_string();

                x.len() == y.len() && x.eq(&y)
            }
            (_, _) => false,
        };

        stack.push(Value::Bool(result));

        InterpretResult::Ok
    }
}
