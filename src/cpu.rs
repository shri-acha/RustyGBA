use crate::instructions::{ArthematicTarget, Instruction};
use crate::memory::MemoryBus;

pub struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            //Arithematic Operations
            //Row x8
            Instruction::ADD(target) => {
                match target {
                    ArthematicTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }

                    ArthematicTarget::H => {
                        let value = self.registers.f;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }

                    ArthematicTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.bus.read_byte(address);
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /*To be implemented*/ }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArthematicTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    ArthematicTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.bus.read_byte(address);
                        let new_value = self.addC(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* To be implemented */ }
                }
            }
            _ => { /*To be implemented*/ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let mut flags: FlagsRegister = FlagsRegister::from(self.registers.f);
        flags.zero = new_value == 0;
        flags.subtract = false;
        flags.carry = did_overflow;
        flags.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f = u8::from(flags);
        new_value
    }

    fn add_c(&mut self, value: u8) -> u8 {
        let mut flags: FlagsRegister = FlagsRegister::from(self.registers.f);
        let (temp_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (new_value, overflow) = temp_value.overflowing_add(flags.carry as u8);
        flags.zero = new_value == 0;
        flags.subtract = false;
        flags.carry = overflow || did_overflow;
        flags.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f = u8::from(flags);
        new_value
    }
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

//Could do it better copying and pasting for safety :P

macro_rules! define_register_pair {
    ($get_name:ident,$set_name:ident,$high:ident,$low:ident, $mask:expr) => {
        pub fn $get_name(&self) -> u16 {
            ((self.$high as u16) << 8) | (self.$low as u16 & $mask as u16)
        }
        pub fn $set_name(&mut self, val: u16) {
            self.$high = (val >> 8) as u8;
            self.$low = (val as u8) & ($mask as u8);
        }
    };
}

impl Registers {
    // Registers a,b,c,d,e,f,h,l
    // Virtual Registers af,bc,de,hl
    // Special Registers pc,sp
    define_register_pair!(get_af, set_af, a, f, 0xFF);
    define_register_pair!(get_bc, set_bc, a, f, 0xFF);
    define_register_pair!(get_hl, set_hl, a, f, 0xFF);
    define_register_pair!(get_de, set_de, a, f, 0xF0);
}
// Flags Register is a Special Register that stores special flags that determines the result of an
// operation is that is being done in some instances
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

// 1(zero bit) 1(subtract bit) 1(half carry) 1(carry) {0 0 0 0} - all of these must be  0
const ZERO_FLAG_POSITION: u8 = 7;
const SUBTRACT_FLAG_POSITION: u8 = 6;
const HALF_CARRY_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

// Converts FlagRegister to u8
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_POSITION
            | (if flag.carry { 1 } else { 0 } << CARRY_FLAG_POSITION)
    }
}

// Converts from binary 8 bit to FlagsRegister
impl std::convert::From<u8> for FlagsRegister {
    fn from(val: u8) -> FlagsRegister {
        let zero = ((val >> ZERO_FLAG_POSITION) & 0b1) != 0;
        let subtract = ((val >> SUBTRACT_FLAG_POSITION) & 0b1) != 0;
        let half_carry = ((val >> HALF_CARRY_FLAG_POSITION) & 0b1) != 0;
        let carry = ((val >> CARRY_FLAG_POSITION) & 0b1) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
