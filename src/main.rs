#![deny(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::cpu::{Cpu, Instruction, Registers};
mod cpu;

fn main() {
    let mut memory = [0; 8192];
    let mut cpu = Cpu::default();
    cpu.execute(Instruction::LDA(cpu::LdaTarget::Value(5)), &mut memory);
    cpu.execute(Instruction::LDFA(cpu::LdfaTarget::Addr(0xFFe)), &mut memory);
    println!("{:#?}", cpu);
    println!("{:#?}", &memory[0xFFE]);
}
