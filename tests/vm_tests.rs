/* use super::*;

#[test]
fn verify_vm_addition() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Add, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
fn verify_vm_substraction() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Substract, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
fn verify_vm_multiplication() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Divide, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
fn verify_vm_division() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Divide, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
fn verify_vm_negation() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Negate, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
fn verify_vm_complex_result() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Negate, 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Multiply, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
}

#[test]
#[should_panic]
fn verify_vm_stack_error() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Add, 123);
    assert_eq!(InterpretResult::Ok, VirtualMachine::interpret(chunk));
} */
