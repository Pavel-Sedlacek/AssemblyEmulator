use crate::mem::mem::Memory;

pub struct RAM {
    // 2048b memory (256 * 8b)
    memory: [u8; 256],
}

impl RAM {
    pub fn load(&mut self, program_pointer: u8, data: &[u8]) {
        for i in 0..data.len() {
            self.memory[(program_pointer + i as u8) as usize] = data[i]
        }
    }
}

impl RAM {
    pub fn new() -> Self {
        RAM {
            memory: [0x0; 256]
        }
    }
}

impl Memory for RAM {
    fn fetch_byte(&self, index: u8) -> u8 {
        self.memory[usize::try_from(index).unwrap()]
    }

    fn fetch_word(&self, index: u8) -> u16 {
        let x: u16 = self.memory[usize::try_from(index).unwrap()] as u16;
        (x << 8) + (self.memory[usize::try_from(index + 1).unwrap()] as u16)
    }

    fn write_byte(&mut self, index: u8, byte: u8) {
        self.memory[usize::try_from(index).unwrap()] = byte
    }

    fn write_word(&mut self, index: u8, word: u16) {
        self.memory[usize::try_from(index).unwrap()] = (word >> 8) as u8;
        self.memory[usize::try_from(index + 1).unwrap()] = word as u8;
    }

    fn reset(&mut self) {
        for i in 0..self.memory.len() {
            self.memory[i] = 0x0
        }
    }
}