use crate::op_codes::{OpCode, NMOS_6502_OPCODES_MAP};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        return (hi << 8) | (lo as u16);
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let op_code = NMOS_6502_OPCODES_MAP.get(&code).unwrap(); // TODO: get rid of unwrap

            match op_code.mnemonic {
                "ADC" => {
                    self.adc(&op_code);
                }
                "AND" => {
                    self.and(&op_code);
                }
                "ASL" => {
                    self.asl(&op_code);
                }
                "BCC" => {
                    self.bcc(&op_code);
                }
                "BCS" => {
                    self.bcs(&op_code);
                }
                "BEQ" => {
                    self.beq(&op_code);
                }
                "BIT" => {
                    self.bit(&op_code);
                }
                "BMI" => {
                    self.bmi(&op_code);
                }
                "BNE" => {
                    self.bne(&op_code);
                }
                "BPL" => {
                    self.bpl(&op_code);
                }
                "BRK" => {
                    return;
                }
                "BVC" => {
                    self.bvc(&op_code);
                }
                "BVS" => {
                    self.bvs(&op_code);
                }
                "CLC" => {
                    self.clc(&op_code);
                }
                "CLD" => {
                    self.cld(&op_code);
                }
                "CLI" => {
                    self.cli(&op_code);
                }
                "CLV" => {
                    self.clv(&op_code);
                }
                "CMP" => {
                    self.cmp(&op_code);
                }
                "CPX" => {
                    self.cpx(&op_code);
                }
                "CPY" => {
                    self.cpy(&op_code);
                }
                "DEC" => {
                    self.dec(&op_code);
                }
                "DEX" => {
                    self.dex(&op_code);
                }
                "DEY" => {
                    self.dey(&op_code);
                }
                "EOR" => {
                    self.eor(&op_code);
                }
                "INC" => {
                    self.inc(&op_code);
                }
                "INX" => {
                    self.inx(&op_code);
                }
                "INY" => {
                    self.iny(&op_code);
                }
                "JMP" => {
                    self.jmp(&op_code);
                }
                "JSR" => {
                    self.jsr(&op_code);
                }
                "LDA" => {
                    self.lda(&op_code);
                }
                "LDX" => {
                    self.ldx(&op_code);
                }
                "LDY" => {
                    self.ldy(&op_code);
                }
                "LSR" => {
                    self.lsr(&op_code);
                }
                "NOP" => {
                    self.nop(&op_code);
                }
                "ORA" => {
                    self.ora(&op_code);
                }
                "PHA" => {
                    self.pha(&op_code);
                }
                "PHP" => {
                    self.php(&op_code);
                }
                "PLA" => {
                    self.pla(&op_code);
                }
                "PLP" => {
                    self.plp(&op_code);
                }
                "ROL" => {
                    self.rol(&op_code);
                }
                "ROR" => {
                    self.ror(&op_code);
                }
                "RTI" => {
                    self.rti(&op_code);
                }
                "RTS" => {
                    self.rts(&op_code);
                }
                "SBC" => {
                    self.sbc(&op_code);
                }
                "SEC" => {
                    self.sec(&op_code);
                }
                "SED" => {
                    self.sed(&op_code);
                }
                "SEI" => {
                    self.sei(&op_code);
                }
                "STA" => {
                    self.sta(&op_code);
                }
                "STX" => {
                    self.stx(&op_code);
                }
                "STY" => {
                    self.sty(&op_code);
                }
                "TAX" => {
                    self.tax(&op_code);
                }
                "TAY" => {
                    self.tay(&op_code);
                }
                "TSX" => {
                    self.tsx(&op_code);
                }
                "TXA" => {
                    self.txa(&op_code);
                }
                "TXS" => {
                    self.txs(&op_code);
                }
                "TYA" => {
                    self.tya(&op_code);
                }
                _ => panic!(),
            }

            //match code {
            //    // LDA start
            //    0xA9 => {
            //        self.lda(&AddressingMode::Immediate);
            //        self.program_counter += 1;
            //    }
            //    0xA5 => {
            //        self.lda(&AddressingMode::ZeroPage);
            //        self.program_counter += 1;
            //    }
            //    0xB5 => {
            //        self.lda(&AddressingMode::ZeroPage_X);
            //        self.program_counter += 1;
            //    }
            //    0xAD => {
            //        self.lda(&AddressingMode::Absolute);
            //        self.program_counter += 2;
            //    }
            //    0xBD => {
            //        self.lda(&AddressingMode::Absolute_X);
            //        self.program_counter += 2;
            //    }
            //    0xB9 => {
            //        self.lda(&AddressingMode::Absolute_Y);
            //        self.program_counter += 2;
            //    }
            //    0xA1 => {
            //        self.lda(&AddressingMode::Indirect_X);
            //        self.program_counter += 1;
            //    }
            //    0xB1 => {
            //        self.lda(&AddressingMode::Indirect_Y);
            //        self.program_counter += 1;
            //    }
            //    // LDA end
            //    // STA start
            //    0x85 => {
            //        self.sta(&AddressingMode::ZeroPage);
            //        self.program_counter += 1;
            //    }
            //    0x95 => {
            //        self.sta(&AddressingMode::ZeroPage_X);
            //        self.program_counter += 1;
            //    }
            //    0x8D => {
            //        self.sta(&AddressingMode::Absolute);
            //        self.program_counter += 2;
            //    }
            //    0x9D => {
            //        self.sta(&AddressingMode::Absolute_X);
            //        self.program_counter += 2;
            //    }
            //    0x99 => {
            //        self.sta(&AddressingMode::Absolute_Y);
            //        self.program_counter += 2;
            //    }
            //    0x81 => {
            //        self.sta(&AddressingMode::Indirect_X);
            //        self.program_counter += 1;
            //    }
            //    0x91 => {
            //        self.sta(&AddressingMode::Indirect_Y);
            //        self.program_counter += 1;
            //    }
            //    // STA end
            //    0xAA => self.tax(&AddressingMode::NoneAddressing),
            //    0xE8 => self.inx(&AddressingMode::NoneAddressing),
            //    0x00 => return,
            //    o => panic!("unsupported opscode: {}", o),
            //}
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn update_program_counter(&mut self, op_code_len: u8) {
        match op_code_len {
            2 => self.program_counter += 1,
            3 => self.program_counter += 2,
            _ => panic!(), // TODO: replace panic
        }
    }

    fn adc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn and(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn asl(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bcc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bcs(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn beq(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bit(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bmi(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bne(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bpl(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn brk(&mut self, _op_code: &OpCode) {
        println!("BRK called");
        return;
    }

    fn bvc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bvs(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn clc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn cld(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn cli(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn clv(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn cmp(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn cpx(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn cpy(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn dec(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn dex(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn dey(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn eor(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn inc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn inx(&mut self, _op_code: &OpCode) {
        if self.register_x == 255 {
            self.register_x = 0;
        } else {
            self.register_x += 1;
        }
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn jmp(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn jsr(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn lda(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);

        match op_code.mode {
            AddressingMode::NoneAddressing => (),
            _ => self.update_program_counter(op_code.len),
        }
    }

    fn ldx(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn ldy(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn lsr(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn nop(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn ora(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn pha(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn php(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn pla(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn plp(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn rol(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn ror(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn rti(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn rts(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sbc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sec(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sed(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sei(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sta(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        self.mem_write(addr, self.register_a);

        match op_code.mode {
            AddressingMode::NoneAddressing => panic!(),
            _ => self.update_program_counter(op_code.len),
        }
    }

    fn stx(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn sty(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn tax(&mut self, _op_code: &OpCode) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn tsx(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn txa(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn txs(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn tya(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                return addr;
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                return addr;
            }
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_x as u16);
                return addr;
            }
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_y as u16);
                return addr;
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                return (hi as u16) << 8 | (lo as u16);
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                return deref;
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5); // 5 == 0x05
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0010 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 2); // 2 == 0x02
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10); // 10 == 0x0a
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1); // 193 == 0xc1
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1); // 1 == 0x01
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 85); // 85 == 0x55
    }
}
