use crate::op_codes::{OpCode, NMOS_6502_OPCODES_MAP};
/* Processor status flag outline
    7  bit  0
    7654 3210
    ---- ----
    NV1B DIZC
    |||| ||||
    |||| |||+- Carry
    |||| ||+-- Zero
    |||| |+--- Interrupt Disable
    |||| +---- Decimal
    |||+------ (No CPU effect; see: the B flag)
    ||+------- (No CPU effect; always pushed as 1)
    |+-------- Overflow
    +--------- Negative
*/

#[derive(Debug)]
pub struct Processor {
    carry: u8,
    zero: u8,
    interrupt: u8,
    decimal: u8,
    b: u8,
    _1: u8,
    overflow: u8,
    negative: u8,
}

impl Processor {
    fn new() -> Self {
        return Self {
            carry: 0,
            zero: 0,
            interrupt: 0,
            decimal: 0,
            b: 1,
            _1: 1,
            overflow: 0,
            negative: 0,
        };
    }

    fn set_carry(&mut self) {
        self.carry = 1;
    }

    fn set_zero(&mut self) {
        self.zero = 1;
    }

    fn set_interrupt(&mut self) {
        self.interrupt = 1;
    }

    fn set_decimal(&mut self) {
        self.decimal = 1;
    }

    fn set_overflow(&mut self) {
        self.overflow = 1;
    }

    fn set_negative(&mut self) {
        self.negative = 1;
    }

    fn clear_carry(&mut self) {
        self.carry = 0;
    }

    fn clear_zero(&mut self) {
        self.zero = 0;
    }

    fn clear_interrupt(&mut self) {
        self.interrupt = 0;
    }

    fn clear_decimal(&mut self) {
        self.decimal = 0;
    }

    fn clear_overflow(&mut self) {
        self.overflow = 0;
    }

    fn clear_negative(&mut self) {
        self.negative = 0;
    }
}

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
    Indirect,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16; // lower 8 bits read from current pos
        let hi = self.mem_read(pos + 1) as u16; // upper 8 bits read from next pos
        return (hi << 8) | (lo as u16); // << high is shifted 8 bit positions left and combined
                                        // with low to form complete 16 bit value
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8; // remove the lower 8 bits
        let lo = (data & 0xff) as u8; // 0xff == 255, or 0000000011111111 so only lower 8 bits are
                                      // kept
        self.mem_write(pos, lo); // write low value to current position
        self.mem_write(pos + 1, hi); // write high value to next position
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
    pub status: Processor,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        let status = Processor::new();
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let op_code = NMOS_6502_OPCODES_MAP
                .get(&code)
                .expect("code not recognized"); // TODO: get rid of unwrap

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
            self.update_program_counter(op_code.len);
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = Processor::new();

        self.program_counter = self.mem_read_u16(0xFFFC);
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
        self.program_counter += (op_code_len - 1) as u16;
    }

    fn adc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn and(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);
        self.register_a = self.register_a & data;
        self.update_zero_and_negative_flags(self.register_a);
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

    //fn brk(&mut self, _op_code: &OpCode) { no instructions to carry out

    fn bvc(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn bvs(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn clc(&mut self, op_code: &OpCode) {
        self.status.clear_carry();
    }

    fn cld(&mut self, op_code: &OpCode) {
        self.status.clear_decimal();
    }

    fn cli(&mut self, op_code: &OpCode) {
        self.status.clear_interrupt();
    }

    fn clv(&mut self, op_code: &OpCode) {
        self.status.clear_overflow();
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

    fn dec(&mut self, op_code: &OpCode) -> u8 {
        let addr = self.get_operand_address(&op_code.mode);
        let mut data = self.mem_read(addr);
        if data == 0 {
            data = 255;
        } else {
            data -= 1;
        }
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        return data;
    }

    fn dex(&mut self, op_code: &OpCode) {
        if self.register_x == 0 {
            self.register_x = 255;
        } else {
            self.register_x -= 1;
        }
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self, op_code: &OpCode) {
        if self.register_y == 0 {
            self.register_y = 255;
        } else {
            self.register_y -= 1;
        }
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);
        self.register_a = self.register_a ^ data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inc(&mut self, op_code: &OpCode) -> u8 {
        let addr = self.get_operand_address(&op_code.mode);
        let mut data = self.mem_read(addr);
        if data == 255 {
            data = 0;
        } else {
            data += 1;
        }
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        return data;
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
        if self.register_y == 255 {
            self.register_y = 0;
        } else {
            self.register_y += 1;
        }
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn jmp(&mut self, op_code: &OpCode) {
        let addr = self.mem_read_u16(self.program_counter);
        match op_code.mode {
            AddressingMode::Absolute => {
                self.program_counter = addr;
            }
            AddressingMode::NoneAddressing => {
                let indirect_ref = if addr & 0x00FF == 0x00FF {
                    let lo = self.mem_read(addr);
                    let hi = self.mem_read(addr & 0x00FF);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.mem_read_u16(addr)
                };

                self.program_counter = indirect_ref;
            }
            _ => panic!(),
        }
    }

    fn jsr(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn lda(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);

        self.register_a = data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);

        self.register_x = data;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);

        self.register_y = data;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lsr(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn nop(&mut self, op_code: &OpCode) {}

    fn ora(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        let data = self.mem_read(addr);
        self.register_a = self.register_a | data;
        self.update_zero_and_negative_flags(self.register_a);
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
    }

    fn stx(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, op_code: &OpCode) {
        let addr = self.get_operand_address(&op_code.mode);
        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self, _op_code: &OpCode) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self, op_code: &OpCode) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn txa(&mut self, op_code: &OpCode) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self, op_code: &OpCode) {
        todo!();
    }

    fn tya(&mut self, op_code: &OpCode) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.set_zero();
        } else {
            self.status.clear_zero();
        }

        if result & 0b1000_0000 != 0 {
            self.status.set_negative();
        } else {
            self.status.clear_negative();
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
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
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
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
            AddressingMode::Indirect => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base as u8;
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                if lo == 0x0ff {
                    return (ptr as u16 & 0xff00) << 8 | (lo as u16); // simulate 6502 hardware bug
                } else {
                    return (hi as u16) << 8 | (lo as u16);
                }
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
    fn test_lda_sta_dec() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0, 0xC6, 0]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory[0], 0x04);
    }

    #[test]
    fn test_ldx_dex() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x04, 0xCA]);
        assert_eq!(cpu.register_x, 0x03);
    }

    #[test]
    fn test_ldy_dey() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x05, 0x88]);
        assert_eq!(cpu.register_y, 0x04);
    }

    #[test]
    fn test_lda_sta_inc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0, 0xE6, 0]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory[0], 0x06);
    }

    #[test]
    fn test_ldx_inx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x04, 0xE8]);
        assert_eq!(cpu.register_x, 0x05);
    }

    #[test]
    fn test_ldy_iny() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x04, 0xC8]);
        assert_eq!(cpu.register_y, 0x05);
    }

    #[test]
    fn test_lda_and_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x29, 0x04, 0x8D, 0x00]);
        assert_eq!(cpu.register_a, 0x04);
        assert_eq!(cpu.memory[0], 0x04);
    }

    #[test]
    fn test_lda_eor_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x49, 0x04, 0x8D, 0x01]);
        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory[1], 0x01);
    }

    #[test]
    fn test_lda_ora_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x09, 0x10, 0x8D, 0x02]);
        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.memory[2], 0x15);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5); // 5 == 0x05
        assert!(cpu.status.zero == 0);
        assert!(cpu.status.negative == 0);
    }

    #[test]
    fn test_0xa2_ldx_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 5); // 5 == 0x05
        assert!(cpu.status.zero == 0);
        assert!(cpu.status.negative == 0);
    }

    #[test]
    fn test_0xa0_ldy_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 5); // 5 == 0x05
        assert!(cpu.status.zero == 0);
        assert!(cpu.status.negative == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.zero == 1); // 2 == 0x02
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status.zero == 1); // 2 == 0x02
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status.zero == 1); // 2 == 0x02
    }

    #[test]
    fn test_0xaa_tax_copy_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10); // 10 == 0x0a
    }

    #[test]
    fn test_0xa8_tay_copy_a_to_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xa8, 0x00]);
        assert_eq!(cpu.register_y, 10); // 10 == 0x0a
    }

    #[test]
    fn test_0x8a_txa_copy_x_to_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x0a, 0x8a, 0x00]);
        assert_eq!(cpu.register_a, 10); // 10 == 0x0a
    }

    #[test]
    fn test_0x98_tya_move_y_to_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x0a, 0x98, 0x00]);
        assert_eq!(cpu.register_a, 10); // 10 == 0x0a
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
    fn test_iny_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xa8, 0xc8, 0xc8, 0x00]);

        assert_eq!(cpu.register_y, 1); // 1 == 0x01
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 85); // 85 == 0x55
    }

    #[test]
    fn test_lda_sta_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x11, 0x85, 0x00]);
        assert_eq!(cpu.memory[0], 0x11);
    }

    #[test]
    fn test_ldx_stx_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x12, 0x86, 0x00]);
        assert_eq!(cpu.register_x, 0x12);
        assert_eq!(cpu.memory[0], 0x12);
    }

    #[test]
    fn test_ldy_sty_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x13, 0x84, 0x00]);
        assert_eq!(cpu.register_y, 0x13);
        assert_eq!(cpu.memory[0], 0x13);
    }

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![]);
        let pc = cpu.program_counter; // u16 primitives are copied, not moved
        cpu.load_and_run(vec![0xEA, 0xEA, 0xEA]);
        assert_eq!(cpu.program_counter, pc + 3);
    }

    #[test]
    fn test_reset() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x13, 0x84, 0x00]);
        cpu.reset();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.program_counter, 32768);
    }
}
