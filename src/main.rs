#![deny(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::cpu::Registers;
mod cpu;

fn main() {
    let cpu = Registers::default();
    println!("{:#?}", cpu);
}
