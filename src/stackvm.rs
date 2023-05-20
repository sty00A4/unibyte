use super::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackByteCode {
    None, Halt,
    Jump, JumpIf, JumpIfNot,

    Set, Get, Const,

    Add, Sub, Div, Mul, Mod, Pow, Neg,
    EQ, LT, Not,

    Copy, Drop, Swap
}
impl ByteCode for StackByteCode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackInstr {
    op: StackByteCode,
    addr: usize
}
impl StackInstr {
    pub fn new(op: StackByteCode, addr: usize) -> Self {
        Self { op, addr }
    }
}
impl Instr<StackByteCode, usize> for StackInstr {
    fn opcode(&self) -> &StackByteCode {
        &self.op
    }
    fn args(&self) -> &usize {
        &self.addr
    }
}

pub struct StackVM {
    pub program: Vec<StackInstr>,
    pub consts: Vec<f64>,
    pub ip: usize,
    pub halted: bool,
    pub stack: Vec<f64>,
    pub memory: Vec<f64>
}
impl StackVM {
    pub fn new(program: Vec<StackInstr>, consts: Vec<f64>, memory_size: usize) -> Self {
        let mut memory = vec![];
        for _ in 0..memory_size { memory.push(0.) }
        Self { program, consts, ip: 0, halted: false, stack: vec![], memory }
    }
    pub fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }
    pub fn push(&mut self, value: f64) {
        self.stack.push(value)
    }
}
impl VM<StackInstr, String> for StackVM {
    fn halted(&self) -> bool {
        self.halted
    }
    fn fetch_instr(&mut self) -> &StackInstr {
        let instr = &self.program[self.ip];
        self.ip += 1;
        instr
    }
    fn handle_instr(&mut self, instr: StackInstr) -> Result<(), String> {
        let addr = instr.addr;
        match instr.opcode() {
            StackByteCode::None => {}
            StackByteCode::Halt => { self.halted = true }
            StackByteCode::Jump => { self.ip = addr }
            StackByteCode::JumpIf => if self.pop() != 0. { self.ip = addr }
            StackByteCode::JumpIfNot => if self.pop() == 0. { self.ip = addr }
            StackByteCode::Set => { self.memory[addr] = self.pop() }
            StackByteCode::Get => { self.push(self.memory[addr]) }
            StackByteCode::Const => { self.push(self.consts[addr]) }
            StackByteCode::Add => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left + right);
            }
            StackByteCode::Sub => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left - right);
            }
            StackByteCode::Div => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left / right);
            }
            StackByteCode::Mul => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left * right);
            }
            StackByteCode::Mod => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left % right);
            }
            StackByteCode::Pow => {
                let (right, left) = (self.pop(), self.pop());
                self.push(left.powf(right));
            }
            StackByteCode::Neg => {
                let value = self.pop();
                self.push(-value);
            }
            StackByteCode::EQ => {
                let (right, left) = (self.pop(), self.pop());
                self.push(if left == right { 1. } else { 0. });
            }
            StackByteCode::LT => {
                let (right, left) = (self.pop(), self.pop());
                self.push(if left < right { 1. } else { 0. });
            }
            StackByteCode::Not => {
                let value = self.pop();
                self.push(if value == 0. { 1. } else { 0. });
            }
            StackByteCode::Copy => {
                let value = self.pop();
                self.push(value);
                self.push(value);
            }
            StackByteCode::Drop => {
                self.pop();
            }
            StackByteCode::Swap => {
                let (right, left) = (self.pop(), self.pop());
                self.push(right);
                self.push(left);
            }
        }
        Ok(())
    }
}