use crate::chunk::Chunk;
use crate::chunk::Code;
use crate::error::RuntimeError;
use crate::opcode::OpCode;
use crate::value::object::Object;
use crate::value::Value;
use crate::Line;
use anyhow::Result;
use std::collections::HashMap;

pub struct VirtualMachine {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    current_line: Line,
}

impl VirtualMachine {
    pub fn initialize() -> Self {
        Self {
            // TODO: ADD STACKOVERFLOW ERROR
            stack: Vec::with_capacity(u8::MAX.into()),
            globals: HashMap::new(),
            current_line: 0,
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<()> {
        for Code(opcode, line) in chunk.codes.into_iter() {
            self.current_line = line;

            match opcode {
                OpCode::Return => break,
                OpCode::Pop => self.drop_stack_value()?,
                OpCode::Constant(value) => self.stack.push(value),
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(true.into()),
                OpCode::False => self.stack.push(false.into()),
                OpCode::Add => self.execute_addition()?,
                OpCode::Substract | OpCode::Multiply | OpCode::Divide => self.execute_binary_operation(opcode)?,
                OpCode::Negate => self.execute_number_negation()?,
                OpCode::Equal => self.verify_equality()?,
                OpCode::Less | OpCode::Greater => self.interpret_binary_boolean_operation(opcode)?,
                OpCode::Print => self.print_value(),
                OpCode::Not => self.execute_boolean_negation()?,
                OpCode::DefineGlobal(identifier) => self.define_global_variable(identifier),
                OpCode::GetGlobal(identifier) => self.get_global_variable(identifier)?,
                OpCode::SetGlobal(identifier) => self.set_global_variable(identifier)?,
            };
        }

        Ok(())
    }

    fn execute_binary_operation(&mut self, opcode: OpCode) -> Result<()> {
        let (Some(Value::Number(b)), Some(Value::Number(a))) = (self.stack.pop(), self.stack.pop()) else {
            return Err(RuntimeError::ExpectedNumber(self.current_line).into());
        };

        let result = match opcode {
            OpCode::Substract => a - b,
            OpCode::Multiply => a * b,
            OpCode::Divide => a / b,
            _ => unreachable!(),
        };

        self.stack.push(result.into());

        Ok(())
    }

    fn execute_number_negation(&mut self) -> Result<()> {
        let Some(Value::Number(number)) = self.stack.pop() else {
            return Err(RuntimeError::ExpectedNumber(self.current_line).into());
        };

        self.stack.push((-number).into());
        Ok(())
    }

    fn execute_boolean_negation(&mut self) -> Result<()> {
        let is_falsey = match self.stack.pop() {
            Some(Value::Nil) | Some(Value::Bool(false)) => true,
            _ => false,
        };

        self.stack.push(is_falsey.into());
        Ok(())
    }

    fn execute_addition(&mut self) -> Result<()> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => self.stack.push(Value::from(a + b)),
            (Some(Value::Object(Object::Str(a))), Some(Value::Object(Object::Str(b)))) => self.stack.push(format!("{a}{b}").into()),
            _ => return Err(RuntimeError::ExpectedNumberOrString(self.current_line).into()),
        };

        Ok(())
    }

    fn drop_stack_value(&mut self) -> Result<()> {
        self.stack.pop();
        Ok(())
    }

    fn interpret_binary_boolean_operation(&mut self, opcode: OpCode) -> Result<()> {
        let (Some(Value::Number(b)), Some(Value::Number(a))) = (self.stack.pop(), self.stack.pop()) else {
            return Err(RuntimeError::ExpectedNumber(self.current_line).into());
        };

        let result = match opcode {
            OpCode::Less => a < b,
            OpCode::Greater => a > b,
            _ => unreachable!(),
        };

        self.stack.push(result.into());

        Ok(())
    }

    fn verify_equality(&mut self) -> Result<()> {
        let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) else {
            return Err(RuntimeError::ExpectedValue(self.current_line).into());
        };

        let result = match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Object(Object::Str(x)), Value::Object(Object::Str(y))) => x.eq(&y),
            _ => false,
        };

        self.stack.push(result.into());

        Ok(())
    }

    fn print_value(&mut self) {
        println!("{}", self.stack.pop().unwrap_or_default());
    }

    fn define_global_variable(&mut self, identifier: String) {
        self.globals.insert(identifier, self.stack.pop().unwrap_or_default());
    }

    fn get_global_variable(&mut self, identifier: String) -> Result<()> {
        match self.globals.get(&identifier) {
            Some(value) => self.stack.push(value.to_owned()),
            None => return Err(RuntimeError::UndefinedVariable(identifier, self.current_line).into()),
        }

        Ok(())
    }

    fn set_global_variable(&mut self, identifier: String) -> Result<()> {
        match self.globals.get_mut(&identifier) {
            Some(value) => *value = self.stack.pop().unwrap_or_default(),
            None => return Err(RuntimeError::UndefinedVariable(identifier, self.current_line).into()),
        }

        Ok(())
    }
}
