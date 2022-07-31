use crate::cpu::cpu::CPU;
use crate::mem::mem::Memory;

// 1 byte instructions, 1 byte registers
pub struct SimpleCpu {
    a_reg: u8,
    x_reg: u8,
    y_reg: u8,

    // Negative, Overflow, -, Break, Decimal, Interrupt, Zero, Carry
    p_reg: u8,

    stack_pointer: u8,
    program_counter: u8,
}

impl SimpleCpu {
    pub fn new() -> Self {
        SimpleCpu {
            a_reg: 0x0,
            x_reg: 0x0,
            y_reg: 0x0,
            p_reg: 0x0,
            stack_pointer: 0x0,
            program_counter: 0b0001_0000,
        }
    }

    fn inc_pc(&mut self) -> u8 {
        self.program_counter += 1;
        self.program_counter - 1
    }

    fn inc_sp(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.stack_pointer - 1
    }

    fn dec_sp(&mut self) -> u8 {
        self.stack_pointer -= 1;
        self.stack_pointer
    }

    // load accumulator
    pub const LDA: u8 = 0x01;
    // load x
    pub const LDX: u8 = 0x02;
    // load y
    pub const LDY: u8 = 0x03;

    // store accumulator in memory
    pub const STA: u8 = 0x04;
    // store x in memory
    pub const STX: u8 = 0x05;
    // store y in memory
    pub const STY: u8 = 0x06;

    // transfer accumulator to x
    pub const TAX: u8 = 0x07;
    // transfer accumulator to y
    pub const TAY: u8 = 0x08;
    // transfer stack pointer to x
    pub const TSX: u8 = 0x09;
    // transfer x to accumulator
    pub const TXA: u8 = 0x0a;
    // transfer x to stack pointer
    pub const TXS: u8 = 0x0b;
    // transfer y to accumulator
    pub const TYA: u8 = 0x0c;

    // push accumulator to stack
    pub const PHA: u8 = 0x0d;
    // push process status register to stack
    pub const PHP: u8 = 0x0e;
    // pull accumulator from stack
    pub const PLA: u8 = 0x0f;
    // pull process status register from stack
    pub const PLP: u8 = 0x10;

    // decrement accumulator
    pub const DEC: u8 = 0x11;
    // decrement x
    pub const DEX: u8 = 0x12;
    // decrement y
    pub const DEY: u8 = 0x13;
    // increment accumulator
    pub const INC: u8 = 0x14;
    // increment x
    pub const INX: u8 = 0x15;
    // increment y
    pub const INY: u8 = 0x16;

    // compare with accumulator
    pub const CMP: u8 = 0x17;
    // compare with x
    pub const CMX: u8 = 0x18;
    // compare with y
    pub const CMY: u8 = 0x19;

    // branch on equal
    pub const BEQ: u8 = 0x1a;
    // branch on not equal
    pub const BNE: u8 = 0x1b;

    // jump
    pub const JMP: u8 = 0x1c;
    // jump subroutine
    pub const JSR: u8 = 0x1d;
    // return from subroutine
    pub const RTS: u8 = 0x1e;

    // prints A register
    pub const OUA: u8 = 0x1f;

    // exits program
    pub const HLT: u8 = 0x20;

    // add Y to A
    pub const APY: u8 = 0x21;
    // add X to A
    pub const APX: u8 = 0x22;
    pub const APA: u8 = 0x23;

    // add to A
    pub const ADA: u8 = 0x24;
    // add to X
    pub const ADX: u8 = 0x25;
    // add to Y
    pub const ADY: u8 = 0x26;
}

impl CPU for SimpleCpu {
    fn execute(&mut self, opcode: u8, memory: &mut dyn Memory) {
        match opcode {
            0x0 => return,
            // register loading
            SimpleCpu::LDA => self.a_reg = memory.fetch_byte(self.inc_pc()),
            SimpleCpu::LDX => self.x_reg = memory.fetch_byte(self.inc_pc()),
            SimpleCpu::LDY => self.y_reg = memory.fetch_byte(self.inc_pc()),

            // registry storing
            SimpleCpu::STA => memory.write_byte(memory.fetch_byte(self.inc_pc()), self.a_reg),
            SimpleCpu::STX => memory.write_byte(memory.fetch_byte(self.inc_pc()), self.x_reg),
            SimpleCpu::STY => memory.write_byte(memory.fetch_byte(self.inc_pc()), self.y_reg),

            // register transferring
            SimpleCpu::TXA => self.a_reg = self.x_reg,
            SimpleCpu::TYA => self.a_reg = self.y_reg,
            SimpleCpu::TAX => self.x_reg = self.a_reg,
            SimpleCpu::TAY => self.y_reg = self.a_reg,
            SimpleCpu::TSX => self.x_reg = self.stack_pointer,
            SimpleCpu::TXS => self.stack_pointer = self.x_reg,

            // push-pulling
            SimpleCpu::PHA => {
                memory.write_byte(self.inc_sp(), self.a_reg);
            }
            SimpleCpu::PHP => memory.write_byte(self.inc_sp(), self.p_reg),
            SimpleCpu::PLA => self.a_reg = memory.fetch_byte(self.dec_sp()),
            SimpleCpu::PLP => self.p_reg = memory.fetch_byte(self.dec_sp()),

            // increment, decrement
            SimpleCpu::DEC => self.a_reg -= 1,
            SimpleCpu::DEX => self.x_reg -= 1,
            SimpleCpu::DEY => self.y_reg -= 1,
            SimpleCpu::INC => self.a_reg += 1,
            SimpleCpu::INX => self.x_reg += 1,
            SimpleCpu::INY => self.y_reg += 1,

            // comparing
            SimpleCpu::CMP => self.p_reg = self.p_reg | ((self.a_reg == memory.fetch_byte(self.inc_pc())) as u8) << 1,
            SimpleCpu::CMX => self.p_reg = self.p_reg | ((self.x_reg == memory.fetch_byte(self.inc_pc())) as u8) << 1,
            SimpleCpu::CMY => self.p_reg = self.p_reg | ((self.y_reg == memory.fetch_byte(self.inc_pc())) as u8) << 1,

            // branching
            SimpleCpu::BEQ => {
                let p = self.inc_pc();
                if (self.p_reg >> 1) & 0x1 == 0x1 {
                    self.program_counter = memory.fetch_byte(p);
                };
                self.p_reg &= !(1 << 1)
            }
            SimpleCpu::BNE => {
                let p = self.inc_pc();
                if (self.p_reg >> 1) & 0x1 == 0x0 {
                    self.program_counter = memory.fetch_byte(p);
                };
                self.p_reg &= !(1 << 1)
            }

            // jumping
            SimpleCpu::JMP => self.program_counter = memory.fetch_byte(self.inc_pc()),
            SimpleCpu::JSR => {
                self.execute(SimpleCpu::PHP, memory);
                self.execute(SimpleCpu::PHA, memory);
                self.a_reg = self.program_counter;
                self.execute(SimpleCpu::PHA, memory);
                self.execute(SimpleCpu::TXA, memory);
                self.execute(SimpleCpu::PHA, memory);
                self.execute(SimpleCpu::TYA, memory);
                self.execute(SimpleCpu::PHA, memory);
                self.program_counter = memory.fetch_byte(self.inc_pc())
            }
            SimpleCpu::RTS => {
                self.execute(SimpleCpu::PLA, memory);
                self.execute(SimpleCpu::TAY, memory);
                self.execute(SimpleCpu::PLA, memory);
                self.execute(SimpleCpu::TAX, memory);
                self.execute(SimpleCpu::PLA, memory);
                self.program_counter = self.a_reg;
                self.execute(SimpleCpu::PLA, memory);
                self.execute(SimpleCpu::PLP, memory)
            }

            SimpleCpu::HLT => {
                self.program_counter = 0b1111_1111
            }

            SimpleCpu::APY => self.a_reg += self.y_reg,
            SimpleCpu::APX => self.a_reg += self.x_reg,
            SimpleCpu::APA => self.a_reg += self.a_reg,

            SimpleCpu::ADA => self.a_reg += memory.fetch_byte(self.inc_pc()),
            SimpleCpu::ADX => self.x_reg += memory.fetch_byte(self.inc_pc()),
            SimpleCpu::ADY => self.y_reg += memory.fetch_byte(self.inc_pc()),

            SimpleCpu::OUA => {
                println!("A: {}", self.a_reg)
            }

            _ => return
        };
    }

    fn launch(&mut self, memory: &mut dyn Memory) {
        while self.program_counter < 0b1111_1111 {
            let sp = self.inc_pc();
            self.execute(memory.fetch_byte(sp), memory);

            //         let mut buffer = String::new();
            //         stdin().read_line(&mut buffer);
            //         println!("CPU:
            // Accumulator Register:      {}, ({})
            // X Register:                {}, ({})
            // Y Register:                {}, ({})
            // Processor Flag Register:   {}, ({})
            // Stack Pointer:             {}, ({})
            // Program Counter:           {}, ({})",
            //                  self.a_reg, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.a_reg)).as_str(),
            //                  self.x_reg, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.x_reg)).as_str(),
            //                  self.y_reg, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.y_reg)).as_str(),
            //                  self.p_reg, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.p_reg)).as_str(),
            //                  self.stack_pointer, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.stack_pointer)).as_str(),
            //                  self.program_counter, "0x".to_owned() + format!("{:0>8}", format!("{:b}", self.program_counter)).as_str()
            //         )
        }
    }
}