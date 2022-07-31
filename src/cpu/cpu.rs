use crate::mem::mem::Memory;

pub trait CPU {
    fn execute(&mut self, opcode: u8, memory: &mut dyn Memory);
    fn launch(&mut self, memory: &mut dyn Memory);
}