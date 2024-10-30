use crate::cpu::AddressingMode;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    pub fn new(
        code: u8,
        mnemonic: &'static str,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        return Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        };
    }
}

lazy_static! {
    pub static ref NMOS_6502_OPCODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),

        OpCode::new(0xA9, "INX", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X), //cycles + 1 if page crossed
        OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y), //cycles + 1 if page crossed
        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y), //cycles + 1 if page crossed

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0x91, "TAX", 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref NMOS_6502_OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut opcodes_map = HashMap::new();
        for cpu_op in &*NMOS_6502_OPCODES {
            opcodes_map.insert(cpu_op.code, cpu_op);
        }

        return opcodes_map;
    };
}
