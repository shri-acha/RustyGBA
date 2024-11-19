pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

//Could do it better copying and pasting for safety :P

macro_rules! define_register_pair {
    ($get_name:ident,$set_name:ident,$high:ident,$low:ident, $mask:expr) => {
        pub fn $get_name(&self) -> u16 {
        ((self.$high as u16)<<8) | (self.$low as u16 & $mask as u16)
        }
        pub fn $set_name(&mut self, val: u16){
            self.$high = (val >> 8 )as u8;
            self.$low = (val as u8) & ($mask as u8);
        }
    };
}

impl Registers {
    // Registers a,b,c,d,e,f,h,l
    // Virtual Registers af,bc,de,hl
    // Special Registers pc,sp
    define_register_pair!(get_af,set_af,a,f,0xFF);
    define_register_pair!(get_bc,set_bc,a,f,0xFF);
    define_register_pair!(get_hl,set_hl,a,f,0xFF);
    define_register_pair!(get_de,set_de,a,f,0xF0);
}

struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_FLAG_POSITION:u8 = 7;
const SUBTRACT_FLAG_POSITION:u8 = 6;
const HALF_CARRY_FLAG_POSITION:u8 = 5;
const CARRY_FLAG_POSITION:u8 = 4;



impl std::convert::From<FlagsRegister> for u8{
    fn from(flag: FlagsRegister) ->u8 {
    (if flag.zero {1} else {0}) << ZERO_FLAG_POSITION |
    (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_POSITION | 
    (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_POSITION | 
    (if flag.carry {1} else {0} << CARRY_FLAG_POSITION)
    }
}

impl std::convert::From<u8> for FlagsRegister{
    fn from(val: u8) -> FlagsRegister{
        let zero = ((val >> ZERO_FLAG_POSITION) & 0b1)!=0;
        let subtract = ((val >> SUBTRACT_FLAG_POSITION) & 0b1)!=0;
        let  half_carry = ((val >> HALF_CARRY_FLAG_POSITION) & 0b1)!=0;
        let carry = ((val >> CARRY_FLAG_POSITION) & 0b1)!=0;
        FlagsRegister{ 
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
