use std::convert::From;

enum Instruction {

}

impl Instruction {
    // decode happens here !!
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            _ => None,
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f.clone()) as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[derive(Clone)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl From<u8> for FlagsRegister {
    fn from(value: u8) -> Self {
        FlagsRegister {
            zero: ((value >> 7) & 1) != 0,
            subtract: ((value >> 6) & 1) != 0,
            half_carry: ((value >> 5) & 1) != 0,
            carry: ((value >> 4) & 1) != 0,
        }
    }
}

impl From<FlagsRegister> for u8 {
    fn from(value: FlagsRegister) -> Self {
        (if value.zero { 1 } else { 0 }) << 7 |
        (if value.subtract { 1 } else { 0 }) << 6 |
        (if value.half_carry { 1 } else { 0 }) << 5 |
        (if value.carry { 1 } else { 0 }) << 4
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

struct CPU {
    registers: Registers,
    bus: MemoryBus,
    pc: u16,
    sp: u16,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            _ => todo!(),
        }
    }
}
