use crate::chunk::op_code::OpCode::{self, *};
use crate::chunk::Chunk;
use crate::chunk::Line;
use crate::compiler::compile;
use crate::value::Value;
use std::collections::HashMap;


#[derive(Debug)]
pub struct VirtualMachine {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

#[derive(PartialEq, Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VirtualMachine {
    pub fn initialize() -> Self {
        Self {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, source: &str, debug_chunk: bool) -> InterpretResult {
        let mut chunk = Chunk::create();

        if !compile(&mut chunk, source) {
            return InterpretResult::CompileError;
        }

        if debug_chunk {
            println!("{:?}", chunk);
        }

        self.run(chunk)
    }

    fn run(&mut self, chunk: Chunk) -> InterpretResult {
        for (op_code, line) in chunk.iter() {
            match op_code {
                Return => return InterpretResult::Ok,
                Constant(constant) => self.stack.push(constant.clone()),
                Add => {
                    if self.interpret_addition() == InterpretResult::RuntimeError {
                        return self.throw_error("Operands must be a number or strings.", *line);
                    }
                }
                Substract | Multiply | Divide => {
                    if self.interpret_binary_operation(op_code) == InterpretResult::RuntimeError {
                        return self.throw_error("Operands must be a number.", *line);
                    }
                }
                Negate => {
                    if let Some(Value::Number(number)) = self.stack.pop() {
                        self.stack.push(Value::Number(-number));
                    } else {
                        self.throw_error("Operands must be a number.", *line);
                    }
                }
                Nil => self.stack.push(Value::Nil),
                True => self.stack.push(Value::from(true)),
                False => self.stack.push(Value::from(false)),
                Not => {
                    let is_falsey = self.is_falsey();
                    self.stack.push(Value::from(is_falsey));
                }
                Equal => {
                    if self.interpret_values_equal() == InterpretResult::RuntimeError {
                        return self.throw_error("Operand must be a number.", *line);
                    }
                }
                Greater | Less => {
                    if self.interpret_binary_boolean_operation(op_code) == InterpretResult::RuntimeError {
                        return self.throw_error("Operand must be a number.", *line);
                    }
                }
                Print => {
                    if let Some(value) = self.stack.pop() {
                        println!("{value}")
                    }
                }
                Pop => {
                    self.stack.pop();
                }
                DefineGlobal => {
                    if let (Some(name), Some(value)) = (self.stack.pop(), self.stack.pop()) {
                        self.globals.insert(name.to_string(), value);
                    }
                }
                GetGlobal => {
                    if let Some(name) = self.stack.pop() {
                        match self.globals.get(&name.to_string()) {
                            Some(value) => self.stack.push(value.clone()),
                            None => return self.throw_error(&format!("Undefined variable '{}'", name), *line),
                        }
                    }
                }

                SetGlobal => {
                    if let Some(name) = self.stack.pop() {
                        match self.globals.get_mut(&name.to_string()) {
                            // TODO: Check and verify the unwrap.
                            Some(value) => *value = self.stack.pop().unwrap(),
                            None => return self.throw_error(&format!("Undefined variable '{}'", name), *line),
                        }
                    }
                }
            };
        }

        InterpretResult::Ok
    }

    fn interpret_binary_operation(&mut self, code: &OpCode) -> InterpretResult {
        if let (Some(Value::Number(b)), Some(Value::Number(a))) = (self.stack.pop(), self.stack.pop()) {
            let result = match code {
                Substract => a - b,
                Multiply => a * b,
                Divide => a / b,
                _ => unreachable!(),
            };

            self.stack.push(Value::Number(result));
            InterpretResult::Ok
        } else {
            InterpretResult::RuntimeError
        }
    }

    fn interpret_addition(&mut self) -> InterpretResult {
        let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) else {
            return InterpretResult::RuntimeError;
        };

        if let (Value::Number(a), Value::Number(b)) = (&a, &b) {
            self.stack.push(Value::from(a + b));
        }

        if let (Value::Object(a), Value::Object(b)) = (&a, &b) {
            return self.concatenate(a.to_string(), b.to_string());
        }

        InterpretResult::Ok
    }

    fn concatenate(&mut self, mut a: String, b: String) -> InterpretResult {
        a.push_str(&b);
        self.stack.push(Value::from(a));
        InterpretResult::Ok
    }

    fn interpret_binary_boolean_operation(&mut self, code: &OpCode) -> InterpretResult {
        let b = match self.stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return InterpretResult::RuntimeError,
        };

        let a = match self.stack.pop() {
            Some(Value::Number(number)) => number,
            _ => return InterpretResult::RuntimeError,
        };

        let result = match code {
            Less => a < b,
            Greater => a > b,
            _ => unreachable!(),
        };

        self.stack.push(Value::Bool(result));

        InterpretResult::Ok
    }

    fn throw_error(&self, message: &str, line: Line) -> InterpretResult {
        eprintln!("{} [line {}] in script.", message, line);
        InterpretResult::RuntimeError
    }

    // TODO: REMOVE THE STACK POP AND PASS IT AS A PARAMTER
    /// Checks if value is falsy. Nil and false values are falsy and every other value behaves like true.
    fn is_falsey(&mut self) -> bool {
        match self.stack.pop() {
            Some(Value::Nil) | Some(Value::Bool(false)) => true,
            _ => false,
        }
    }

    // TODO: Refactor this code.
    fn interpret_values_equal(&mut self) -> InterpretResult {
        let b = match self.stack.pop() {
            Some(value) => value,
            _ => return InterpretResult::RuntimeError,
        };

        let a = match self.stack.pop() {
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

        self.stack.push(Value::Bool(result));

        InterpretResult::Ok
    }
}
