pub enum Instruction{
    ADD(ArthematicTarget), // Adds to specific register
    ADDHL(ArthematicTarget), // Adds to HL register
    ADC(ArthematicTarget), // Comments to be added
    SUB(ArthematicTarget),
    SBC(ArthematicTarget),
    AND(ArthematicTarget),
    OR(ArthematicTarget),
    XOR(ArthematicTarget),
    CP(ArthematicTarget),
    INC(ArthematicTarget),
    DEC(ArthematicTarget),
    CCF(ArthematicTarget),
    SCF(ArthematicTarget),
    RRA(),
    RLA(),
    RRCA(),
    RRLA(),
    CPL(),
    BIT(ArthematicTarget),
    RES(ArthematicTarget),
    SET(ArthematicTarget),
    SRL(ArthematicTarget),
    RR(ArthematicTarget),
    RRC(ArthematicTarget),
    RLC(ArthematicTarget),
    SRA(ArthematicTarget),
    SLA(ArthematicTarget),
    SWAP(ArthematicTarget),
}

pub enum ArthematicTarget{
    A,B,C,D,E,H,L,
    HL,
    Immediate(u8),
}
