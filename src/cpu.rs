use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct CpuFlags: u8 {
        const ZERO = 0b1000_0000;
        const SUBSTRACTION = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
        const _ = !0;
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: CpuFlags,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

macro_rules! reg16 {
    ($reg:ident, $set_reg:ident, $reg1:ident, $reg2:ident) => {
        pub fn $reg(&self) -> u16 {
            ((self.$reg1 as u16) << 8) | self.$reg2 as u16
        }

        pub fn $set_reg(&mut self, value: u16) {
            self.$reg1 = ((value & 0xFF00) >> 8) as u8;
            self.$reg2 = (value & 0x00FF) as u8;
        }
    };
}

impl Registers {
    reg16!(bc, set_bc, b, c);
    reg16!(de, set_de, d, e);
    reg16!(hl, set_hl, h, l);

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f.bits() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = CpuFlags::from_bits_truncate((value & 0x00FF) as u8);
    }
}

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn ldn(&mut self, target: LdnTarget, value: u8) {
        match target {
            LdnTarget::A => {
                self.registers.a = value;
            }
            LdnTarget::B => {
                self.registers.b = value;
            }
            LdnTarget::C => {
                self.registers.c = value;
            }
            LdnTarget::D => {
                self.registers.d = value;
            }
            LdnTarget::E => {
                self.registers.e = value;
            }
            LdnTarget::H => {
                self.registers.h = value;
            }
            LdnTarget::L => {
                self.registers.l = value;
            }
            LdnTarget::BC => {
                self.registers.set_bc(value as u16);
            }
            LdnTarget::DE => {
                self.registers.set_de(value as u16);
            }
            LdnTarget::HL => {
                self.registers.set_hl(value as u16);
            }
        }
    }

    fn ldrr(&mut self, to: LdrrTarget, from: LdrrTarget, memory: &mut [u8]) {
        let value = match from {
            LdrrTarget::A => self.registers.a as u16,
            LdrrTarget::B => self.registers.b as u16,
            LdrrTarget::C => self.registers.c as u16,
            LdrrTarget::D => self.registers.d as u16,
            LdrrTarget::E => self.registers.e as u16,
            LdrrTarget::H => self.registers.h as u16,
            LdrrTarget::L => self.registers.l as u16,
            LdrrTarget::HL => memory[self.registers.hl() as usize] as u16,
        };

        match to {
            LdrrTarget::A => self.registers.a = (value & 0x00FF) as u8,
            LdrrTarget::B => self.registers.b = (value & 0x00FF) as u8,
            LdrrTarget::C => self.registers.c = (value & 0x00FF) as u8,
            LdrrTarget::D => self.registers.d = (value & 0x00FF) as u8,
            LdrrTarget::E => self.registers.e = (value & 0x00FF) as u8,
            LdrrTarget::H => self.registers.h = (value & 0x00FF) as u8,
            LdrrTarget::L => self.registers.l = (value & 0x00FF) as u8,
            LdrrTarget::HL => memory[self.registers.hl() as usize] = (value & 0x00FF) as u8,
        };
    }

    fn lda(&mut self, from: LdaTarget, memory: &mut [u8]) {
        let value = match from {
            LdaTarget::A => self.registers.a as u16,
            LdaTarget::B => self.registers.b as u16,
            LdaTarget::C => self.registers.c as u16,
            LdaTarget::D => self.registers.d as u16,
            LdaTarget::E => self.registers.e as u16,
            LdaTarget::H => self.registers.h as u16,
            LdaTarget::L => self.registers.l as u16,
            LdaTarget::BC => memory[self.registers.bc() as usize] as u16,
            LdaTarget::DE => memory[self.registers.de() as usize] as u16,
            LdaTarget::HL => memory[self.registers.hl() as usize] as u16,
            LdaTarget::Addr(addr) => memory[addr as usize] as u16,
            LdaTarget::Value(value) => value as u16,
        };

        self.registers.a = (value & 0x00FF) as u8;
    }

    fn ldfa(&mut self, to: LdfaTarget, memory: &mut [u8]) {
        match to {
            LdfaTarget::A => self.registers.a = self.registers.a,
            LdfaTarget::B => self.registers.b = self.registers.a,
            LdfaTarget::C => self.registers.c = self.registers.a,
            LdfaTarget::D => self.registers.d = self.registers.a,
            LdfaTarget::E => self.registers.e = self.registers.a,
            LdfaTarget::H => self.registers.h = self.registers.a,
            LdfaTarget::L => self.registers.l = self.registers.a,
            LdfaTarget::BC => memory[self.registers.bc() as usize] = self.registers.a,
            LdfaTarget::DE => memory[self.registers.de() as usize] = self.registers.a,
            LdfaTarget::HL => memory[self.registers.hl() as usize] = self.registers.a,
            LdfaTarget::Addr(addr) => memory[addr as usize] = self.registers.a,
        };
    }

    fn push(&mut self, target: StackTarget, memory: &mut [u8]) {
        let value = match target {
            StackTarget::AF => self.registers.af(),
            StackTarget::BC => self.registers.bc(),
            StackTarget::DE => self.registers.de(),
            StackTarget::HL => self.registers.hl(),
        };

        memory[self.registers.sp as usize] = ((value & 0xFF00) >> 8) as u8;
        self.registers.sp -= 1;
        memory[self.registers.sp as usize] = (value & 0x00FF) as u8;
        self.registers.sp -= 1;
    }

    fn pop(&mut self, target: StackTarget, memory: &mut [u8]) {
        let mut value: u16 = 0;
        self.registers.sp += 1;
        value += memory[self.registers.sp as usize] as u16;
        self.registers.sp += 1;
        value += (memory[self.registers.sp as usize] as u16) << 8;
    }

    fn add(&mut self, target: AddTarget, memory: &mut [u8]) {
        match target {
            AddTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                let (new_value, overflow) = self.registers.a.overflowing_add(value);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags.set(CpuFlags::CARRY, true);
                }

                if new_value == 0 {
                    flags.set(CpuFlags::ZERO, true);
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags.set(CpuFlags::HALF_CARRY, true);
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
            AddTarget::Value(value) => {
                let (new_value, overflow) = self.registers.a.overflowing_add(value);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags.set(CpuFlags::CARRY, true);
                }

                if new_value == 0 {
                    flags.set(CpuFlags::ZERO, true);
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags.set(CpuFlags::HALF_CARRY, true);
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
            other => {
                let value = match other {
                    AddTarget::A => self.registers.a,
                    AddTarget::B => self.registers.b,
                    AddTarget::C => self.registers.c,
                    AddTarget::D => self.registers.d,
                    AddTarget::E => self.registers.e,
                    AddTarget::H => self.registers.h,
                    AddTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let (new_value, overflow) = self.registers.a.overflowing_add(value);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags.set(CpuFlags::CARRY, true);
                }

                if new_value == 0 {
                    flags.set(CpuFlags::ZERO, true);
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags.set(CpuFlags::HALF_CARRY, true);
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
        }
    }

    fn adc(&mut self, target: AddTarget, memory: &mut [u8]) {
        match target {
            AddTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                let (new_value, overflow) = self.registers.a.overflowing_add((value & 0xFF) as u8);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value + self.registers.f.contains(CpuFlags::CARRY) as u8;
            }
            AddTarget::Value(value) => {
                let (new_value, overflow) = self.registers.a.overflowing_add(value);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value + self.registers.f.contains(CpuFlags::CARRY) as u8;
            }
            other => {
                let value = match other {
                    AddTarget::A => self.registers.a,
                    AddTarget::B => self.registers.b,
                    AddTarget::C => self.registers.c,
                    AddTarget::D => self.registers.d,
                    AddTarget::E => self.registers.e,
                    AddTarget::H => self.registers.h,
                    AddTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let (new_value, overflow) = self.registers.a.overflowing_add(value);
                let mut flags = CpuFlags::empty();

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a <= 0xF || value <= 0xF) && new_value > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value + self.registers.f.contains(CpuFlags::CARRY) as u8;
            }
        }
    }

    fn sub(&mut self, target: SubTarget, memory: &mut [u8]) {
        match target {
            SubTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                let (new_value, overflow) = self.registers.a.overflowing_sub((value & 0xFF) as u8);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a > 0xF || value > 0xF) && new_value <= 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
            SubTarget::Value(value) => {
                let (new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a > 0xF || value > 0xF) && new_value <= 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
            other => {
                let value = match other {
                    SubTarget::A => self.registers.a,
                    SubTarget::B => self.registers.b,
                    SubTarget::C => self.registers.c,
                    SubTarget::D => self.registers.d,
                    SubTarget::E => self.registers.e,
                    SubTarget::H => self.registers.h,
                    SubTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let (new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a > 0xF || value > 0xF) && new_value <= 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.f = flags;
                self.registers.a = new_value;
            }
        }
    }

    fn sbc(&mut self, target: SubTarget, memory: &mut [u8]) {
        match target {
            SubTarget::HL => {
                let value = memory[self.registers.hl() as usize]
                    + self.registers.f.contains(CpuFlags::CARRY) as u8;
                let (new_value, overflow) = self.registers.a.overflowing_sub((value & 0xFF) as u8);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) - ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.a = new_value;
            }
            SubTarget::Value(value) => {
                let value = value + self.registers.f.contains(CpuFlags::CARRY) as u8;
                let (new_value, overflow) = self.registers.a.overflowing_sub((value & 0xFF) as u8);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) - ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.a = new_value;
            }
            other => {
                let value = match other {
                    SubTarget::A => self.registers.a,
                    SubTarget::B => self.registers.b,
                    SubTarget::C => self.registers.c,
                    SubTarget::D => self.registers.d,
                    SubTarget::E => self.registers.e,
                    SubTarget::H => self.registers.h,
                    SubTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let value = value + self.registers.f.contains(CpuFlags::CARRY) as u8;
                let (mut new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                    new_value += 1;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) - ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }

                self.registers.a = new_value;
            }
        }
    }

    fn cp(&mut self, target: CpTarget, memory: &mut [u8]) {
        match target {
            CpTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                let (new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) + ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }
            }
            CpTarget::Value(value) => {
                let (new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) + ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }
            }
            other => {
                let value = match other {
                    CpTarget::A => self.registers.a,
                    CpTarget::B => self.registers.b,
                    CpTarget::C => self.registers.c,
                    CpTarget::D => self.registers.d,
                    CpTarget::E => self.registers.e,
                    CpTarget::H => self.registers.h,
                    CpTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let (mut new_value, overflow) = self.registers.a.overflowing_sub(value);
                let mut flags = CpuFlags::empty();
                flags |= CpuFlags::SUBSTRACTION;

                if overflow {
                    flags |= CpuFlags::CARRY;
                    new_value += 1;
                }

                if new_value == 0 {
                    flags |= CpuFlags::ZERO;
                }

                if (self.registers.a & 0xF) + ((value & 0xF) as u8) > 0xF {
                    flags |= CpuFlags::HALF_CARRY;
                }
            }
        }
    }

    fn inc(&mut self, target: IncTarget) {
        match target {
            IncTarget::HL => {
                let value = self.registers.hl();
                let (new_value, overflow) = value.overflowing_add(1);

                let mut flags = self.registers.f.clone();

                if new_value == 0 {
                    flags.set(CpuFlags::ZERO, true);
                }

                if value == 0b0000_0000_0000_1111 {
                    flags.set(CpuFlags::HALF_CARRY, true);
                }

                self.registers.set_hl(new_value);
            }
            other => {
                let value = match other {
                    IncTarget::A => self.registers.a,
                    IncTarget::B => self.registers.b,
                    IncTarget::C => self.registers.c,
                    IncTarget::D => self.registers.d,
                    IncTarget::E => self.registers.e,
                    IncTarget::H => self.registers.h,
                    IncTarget::L => self.registers.l,
                    _ => unreachable!("Other targets must be checked before."),
                };
                let (new_value, overflow) = value.overflowing_add(1);

                let mut flags = self.registers.f.clone();

                if new_value == 0 {
                    flags.set(CpuFlags::ZERO, true);
                }

                if value == 0b0000_1111 {
                    flags.set(CpuFlags::HALF_CARRY, true);
                }

                let value = match other {
                    IncTarget::A => self.registers.a = new_value,
                    IncTarget::B => self.registers.b = new_value,
                    IncTarget::C => self.registers.c = new_value,
                    IncTarget::D => self.registers.d = new_value,
                    IncTarget::E => self.registers.e = new_value,
                    IncTarget::H => self.registers.h = new_value,
                    IncTarget::L => self.registers.l = new_value,
                    _ => unreachable!("Other targets must be checked before."),
                };
            }
        }
    }

    fn and(&mut self, target: LogicTarget, memory: &mut [u8]) {
        match target {
            LogicTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                self.registers.a &= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                flag |= CpuFlags::HALF_CARRY;
                self.registers.f = flag;
            }
            LogicTarget::Value(value) => {
                self.registers.a &= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                flag |= CpuFlags::HALF_CARRY;
                self.registers.f = flag;
            }
            other => {
                let value = match other {
                    LogicTarget::A => self.registers.a,
                    LogicTarget::B => self.registers.b,
                    LogicTarget::C => self.registers.c,
                    LogicTarget::D => self.registers.d,
                    LogicTarget::E => self.registers.e,
                    LogicTarget::H => self.registers.h,
                    LogicTarget::L => self.registers.l,
                    _ => unreachable!("HL and Value must have been checked before."),
                };
                self.registers.a &= value;

                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                flag |= CpuFlags::HALF_CARRY;
                self.registers.f = flag;
            }
        }
    }

    fn or(&mut self, target: LogicTarget, memory: &mut [u8]) {
        match target {
            LogicTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                self.registers.a |= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
            LogicTarget::Value(value) => {
                self.registers.a |= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
            other => {
                let value = match other {
                    LogicTarget::A => self.registers.a,
                    LogicTarget::B => self.registers.b,
                    LogicTarget::C => self.registers.c,
                    LogicTarget::D => self.registers.d,
                    LogicTarget::E => self.registers.e,
                    LogicTarget::H => self.registers.h,
                    LogicTarget::L => self.registers.l,
                    _ => unreachable!("HL and Value must have been checked before."),
                };
                self.registers.a |= value;

                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
        }
    }

    fn xor(&mut self, target: LogicTarget, memory: &mut [u8]) {
        match target {
            LogicTarget::HL => {
                let value = memory[self.registers.hl() as usize];
                self.registers.a ^= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
            LogicTarget::Value(value) => {
                self.registers.a ^= value;
                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
            other => {
                let value = match other {
                    LogicTarget::A => self.registers.a,
                    LogicTarget::B => self.registers.b,
                    LogicTarget::C => self.registers.c,
                    LogicTarget::D => self.registers.d,
                    LogicTarget::E => self.registers.e,
                    LogicTarget::H => self.registers.h,
                    LogicTarget::L => self.registers.l,
                    _ => unreachable!("HL and Value must have been checked before."),
                };
                self.registers.a ^= value;

                let mut flag = CpuFlags::empty();
                if self.registers.a == 0 {
                    flag |= CpuFlags::ZERO;
                }

                self.registers.f = flag;
            }
        }
    }

    fn add_hl(&mut self, target: Add16Target) {
        let value = match target {
            Add16Target::BC => self.registers.bc(),
            Add16Target::DE => self.registers.de(),
            Add16Target::HL => self.registers.hl(),
            Add16Target::SP => self.registers.sp,
        };
        let (new_value, overflow) = self.registers.hl().overflowing_add(value);
        let mut flags = CpuFlags::empty();
        flags.set(CpuFlags::ZERO, self.registers.f.contains(CpuFlags::ZERO));

        if overflow {
            flags |= CpuFlags::CARRY;
        }

        if (self.registers.hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF {
            flags |= CpuFlags::HALF_CARRY;
        }

        self.registers.set_hl(new_value);
    }

    pub fn execute(&mut self, instruction: Instruction, memory: &mut [u8]) {
        match instruction {
            Instruction::LDN(target, value) => {
                self.ldn(target, value);
            }
            Instruction::LDRR(to, from) => {
                self.ldrr(to, from, memory);
            }
            Instruction::LDA(from) => {
                self.lda(from, memory);
            }
            Instruction::LDFA(to) => {
                self.ldfa(to, memory);
            }
            Instruction::PUSH(target) => {
                self.push(target, memory);
            }
            Instruction::POP(target) => {
                self.pop(target, memory);
            }
            Instruction::ADD(target) => {
                self.add(target, memory);
            }
            Instruction::ADC(target) => {
                self.adc(target, memory);
            }
            Instruction::SUB(target) => {
                self.sub(target, memory);
            }
            Instruction::SBC(target) => {
                self.sbc(target, memory);
            }
            Instruction::CP(target) => {
                self.cp(target, memory);
            }
            Instruction::INC(target) => {
                self.inc(target);
            }
            Instruction::AND(target) => {
                self.and(target, memory);
            }
            Instruction::OR(target) => {
                self.or(target, memory);
            }
            Instruction::XOR(target) => {
                self.xor(target, memory);
            }
            Instruction::ADD16(target) => {
                self.add_hl(target);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LDN(LdnTarget, u8),
    LDRR(LdrrTarget, LdrrTarget),
    LDA(LdaTarget),
    LDFA(LdfaTarget),
    PUSH(StackTarget),
    POP(StackTarget),
    ADD(AddTarget),
    ADC(AddTarget),
    SUB(SubTarget),
    SBC(SubTarget),
    CP(CpTarget),
    INC(IncTarget),
    AND(LogicTarget),
    OR(LogicTarget),
    XOR(LogicTarget),
    ADD16(Add16Target),
}

#[derive(Debug, Clone)]
pub enum Add16Target {
    BC,
    DE,
    SP,
    HL,
}

#[derive(Debug, Clone)]
pub enum LdaTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    Addr(u16),
    Value(u8),
}

#[derive(Debug, Clone)]
pub enum LdfaTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    Addr(u16),
}

#[derive(Debug, Clone)]
pub enum AddTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    Value(u8),
}

#[derive(Debug, Clone)]
pub enum IncTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Debug, Clone)]
pub enum CpTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    Value(u8),
}

#[derive(Debug, Clone)]
pub enum SubTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    Value(u8),
}

#[derive(Debug, Clone)]
pub enum StackTarget {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, Clone)]
pub enum LdnTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
}

#[derive(Debug, Clone)]
pub enum LdrrTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Debug, Clone)]
pub enum LogicTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    Value(u8),
}

#[cfg(test)]
mod tests {
    use bitflags::Flags;

    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADD(AddTarget::Value(5)), &mut memory);
        assert_eq!(5, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_add_zero() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADD(AddTarget::Value(0)), &mut memory);
        assert_eq!(0, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADD(AddTarget::Value(1)), &mut memory);
        cpu.execute(Instruction::ADD(AddTarget::Value(255)), &mut memory);
        assert_eq!(0, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_add_half_carry() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADD(AddTarget::Value(16)), &mut memory);
        assert_eq!(16, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_adc() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADC(AddTarget::Value(5)), &mut memory);
        assert_eq!(5, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_adc_zero() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADC(AddTarget::Value(0)), &mut memory);
        assert_eq!(0, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_adc_carry() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADC(AddTarget::Value(1)), &mut memory);
        cpu.execute(Instruction::ADC(AddTarget::Value(255)), &mut memory);
        assert_eq!(1, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_adc_half_carry() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::ADC(AddTarget::Value(16)), &mut memory);
        assert_eq!(16, cpu.registers.a);
        assert!(!cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_sub() {
        let mut cpu = Cpu::default();
        cpu.registers.a = 5;
        let mut memory = [0; 8192];
        cpu.execute(Instruction::SUB(SubTarget::Value(1)), &mut memory);
        assert_eq!(4, cpu.registers.a);
        assert!(cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_sub_zero() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::SUB(SubTarget::Value(0)), &mut memory);
        assert_eq!(0, cpu.registers.a);
        assert!(cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_sub_carry() {
        let mut cpu = Cpu::default();
        let mut memory = [0; 8192];
        cpu.execute(Instruction::SUB(SubTarget::Value(1)), &mut memory);
        assert_eq!(255, cpu.registers.a);
        assert!(cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(!cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }

    #[test]
    fn test_sub_half_carry() {
        let mut cpu = Cpu::default();
        cpu.registers.a = 16;
        let mut memory = [0; 8192];
        cpu.execute(Instruction::SUB(SubTarget::Value(1)), &mut memory);
        assert_eq!(15, cpu.registers.a);
        assert!(cpu.registers.f.contains(CpuFlags::SUBSTRACTION));
        assert!(!cpu.registers.f.contains(CpuFlags::ZERO));
        assert!(!cpu.registers.f.contains(CpuFlags::CARRY));
        assert!(cpu.registers.f.contains(CpuFlags::HALF_CARRY));
    }
}
