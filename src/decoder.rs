use crate::instructions::{ArthematicTarget,Instruction};
pub fn decode(opcode: u8) -> Option<Instruction> {
    match opcode {
        // ADD A, r
        0x87 => Some(Instruction::ADD(ArthematicTarget::A)),
        0x80 => Some(Instruction::ADD(ArthematicTarget::B)),
        0x81 => Some(Instruction::ADD(ArthematicTarget::C)),
        0x82 => Some(Instruction::ADD(ArthematicTarget::D)),
        0x83 => Some(Instruction::ADD(ArthematicTarget::E)),
        0x84 => Some(Instruction::ADD(ArthematicTarget::H)),
        0x85 => Some(Instruction::ADD(ArthematicTarget::L)),
        0x86 => Some(Instruction::ADD(ArthematicTarget::HL)),
        
        // ADC A, r
        0x8F => Some(Instruction::ADC(ArthematicTarget::A)),
        0x88 => Some(Instruction::ADC(ArthematicTarget::B)),
        0x89 => Some(Instruction::ADC(ArthematicTarget::C)),
        0x8A => Some(Instruction::ADC(ArthematicTarget::D)),
        0x8B => Some(Instruction::ADC(ArthematicTarget::E)),
        0x8C => Some(Instruction::ADC(ArthematicTarget::H)),
        0x8D => Some(Instruction::ADC(ArthematicTarget::L)),
        0x8E => Some(Instruction::ADC(ArthematicTarget::HL)),

        // SUB r
        0x97 => Some(Instruction::SUB(ArthematicTarget::A)),
        0x90 => Some(Instruction::SUB(ArthematicTarget::B)),
        0x91 => Some(Instruction::SUB(ArthematicTarget::C)),
        0x92 => Some(Instruction::SUB(ArthematicTarget::D)),
        0x93 => Some(Instruction::SUB(ArthematicTarget::E)),
        0x94 => Some(Instruction::SUB(ArthematicTarget::H)),
        0x95 => Some(Instruction::SUB(ArthematicTarget::L)),
        0x96 => Some(Instruction::SUB(ArthematicTarget::HL)),

        // SBC A, r
        0x9F => Some(Instruction::SBC(ArthematicTarget::A)),
        0x98 => Some(Instruction::SBC(ArthematicTarget::B)),
        0x99 => Some(Instruction::SBC(ArthematicTarget::C)),
        0x9A => Some(Instruction::SBC(ArthematicTarget::D)),
        0x9B => Some(Instruction::SBC(ArthematicTarget::E)),
        0x9C => Some(Instruction::SBC(ArthematicTarget::H)),
        0x9D => Some(Instruction::SBC(ArthematicTarget::L)),
        0x9E => Some(Instruction::SBC(ArthematicTarget::HL)),

        // AND r
        0xA7 => Some(Instruction::AND(ArthematicTarget::A)),
        0xA0 => Some(Instruction::AND(ArthematicTarget::B)),
        0xA1 => Some(Instruction::AND(ArthematicTarget::C)),
        0xA2 => Some(Instruction::AND(ArthematicTarget::D)),
        0xA3 => Some(Instruction::AND(ArthematicTarget::E)),
        0xA4 => Some(Instruction::AND(ArthematicTarget::H)),
        0xA5 => Some(Instruction::AND(ArthematicTarget::L)),
        0xA6 => Some(Instruction::AND(ArthematicTarget::HL)),

        // XOR r
        0xAF => Some(Instruction::XOR(ArthematicTarget::A)),
        0xA8 => Some(Instruction::XOR(ArthematicTarget::B)),
        0xA9 => Some(Instruction::XOR(ArthematicTarget::C)),
        0xAA => Some(Instruction::XOR(ArthematicTarget::D)),
        0xAB => Some(Instruction::XOR(ArthematicTarget::E)),
        0xAC => Some(Instruction::XOR(ArthematicTarget::H)),
        0xAD => Some(Instruction::XOR(ArthematicTarget::L)),
        0xAE => Some(Instruction::XOR(ArthematicTarget::HL)),

        // OR r
        0xB7 => Some(Instruction::OR(ArthematicTarget::A)),
        0xB0 => Some(Instruction::OR(ArthematicTarget::B)),
        0xB1 => Some(Instruction::OR(ArthematicTarget::C)),
        0xB2 => Some(Instruction::OR(ArthematicTarget::D)),
        0xB3 => Some(Instruction::OR(ArthematicTarget::E)),
        0xB4 => Some(Instruction::OR(ArthematicTarget::H)),
        0xB5 => Some(Instruction::OR(ArthematicTarget::L)),
        0xB6 => Some(Instruction::OR(ArthematicTarget::HL)),

        // CP r
        0xBF => Some(Instruction::CP(ArthematicTarget::A)),
        0xB8 => Some(Instruction::CP(ArthematicTarget::B)),
        0xB9 => Some(Instruction::CP(ArthematicTarget::C)),
        0xBA => Some(Instruction::CP(ArthematicTarget::D)),
        0xBB => Some(Instruction::CP(ArthematicTarget::E)),
        0xBC => Some(Instruction::CP(ArthematicTarget::H)),
        0xBD => Some(Instruction::CP(ArthematicTarget::L)),
        0xBE => Some(Instruction::CP(ArthematicTarget::HL)),

        // INC r
        0x3C => Some(Instruction::INC(ArthematicTarget::A)),
        0x04 => Some(Instruction::INC(ArthematicTarget::B)),
        0x0C => Some(Instruction::INC(ArthematicTarget::C)),
        0x14 => Some(Instruction::INC(ArthematicTarget::D)),
        0x1C => Some(Instruction::INC(ArthematicTarget::E)),
        0x24 => Some(Instruction::INC(ArthematicTarget::H)),
        0x2C => Some(Instruction::INC(ArthematicTarget::L)),
        0x34 => Some(Instruction::INC(ArthematicTarget::HL)),

        // DEC r
        0x3D => Some(Instruction::DEC(ArthematicTarget::A)),
        0x05 => Some(Instruction::DEC(ArthematicTarget::B)),
        0x0D => Some(Instruction::DEC(ArthematicTarget::C)),
        0x15 => Some(Instruction::DEC(ArthematicTarget::D)),
        0x1D => Some(Instruction::DEC(ArthematicTarget::E)),
        0x25 => Some(Instruction::DEC(ArthematicTarget::H)),
        0x2D => Some(Instruction::DEC(ArthematicTarget::L)),
        0x35 => Some(Instruction::DEC(ArthematicTarget::HL)),

        _ => None,
    }
}
