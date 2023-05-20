#![allow(dead_code)]
#[cfg(test)]
mod tests;
mod stackvm;

pub trait ByteCode where Self: Sized {
    fn to_byte(self) -> u8 {
        0
    }
    fn from_byte(_byte: u8) -> Option<Self> {
        None
    }
}
pub trait VM<I: Clone, E> {
    fn halted(&self) -> bool;
    fn fetch_instr(&mut self) -> &I;
    fn handle_instr(&mut self, instr: I) -> Result<(), E>;
    fn step(&mut self) -> Result<(), E> {
        let instr = self.fetch_instr().clone();
        self.handle_instr(instr)
    }
    fn run(&mut self) -> Result<(), E> {
        while !self.halted() {
            self.step()?;
        }
        Ok(())
    }
}
pub trait Instr<O: ByteCode, A> {
    fn opcode(&self) -> &O;
    fn args(&self) -> &A;
}
