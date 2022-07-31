use crate::cpu::cpu::CPU;
use crate::cpu::simple_cpu::SimpleCpu;
use crate::mem::ram::RAM;

pub mod cpu;
pub mod mem;

fn main() {
    let mut ram = RAM::new();
    ram.load(
        0b0001_0000,
        &[
            SimpleCpu::LDX, 1,                  // load 1 to X register
            SimpleCpu::LDY, 1,                  // load 1 to Y register
            SimpleCpu::TYA,                     // load 1 to Y register
            SimpleCpu::PHA,                     // load 1 to Y register
            SimpleCpu::TXA,                     // load 1 to Y register
            SimpleCpu::PHA,                     // load 1 to Y register

            SimpleCpu::PLA,                     // get A from stack
            SimpleCpu::TAY,                     // transfer A to Y
            SimpleCpu::PLA,                     // get A from stack
            SimpleCpu::TAX,                     // transfer A to X

            SimpleCpu::LDA, 0,                  // load 0 to A
            SimpleCpu::APY,                     // adds Y to A
            SimpleCpu::APX,                     // adds X to A
            SimpleCpu::OUA,                     // prints A register (not really assembly instruction but hey...)
            SimpleCpu::CMP, 233,                // compare A to 233 (13th fibonacci number)
            SimpleCpu::BNE, 0b0001_0000 + 22,   // skip return if A not equals 233

            SimpleCpu::HLT,                     // end program

            SimpleCpu::PHA,                     // push A to stack
            SimpleCpu::TXA,                     // transfer X to A
            SimpleCpu::PHA,                     // push A to stack
            SimpleCpu::JMP, 0b0001_0000 + 8     // go to first line of subroutine
        ],
    );

    let mut simple_cpu = SimpleCpu::new();

    simple_cpu.launch(&mut ram);
}
