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
    flags: u8,
}

impl Processor {
    pub fn new() -> Self {
        let flags = 0b0011_0000;

        return Self { flags };
    }

    pub fn carry(&self) -> u8 {
        return self.flags >> 0 & 1;
    }

    pub fn zero(&self) -> u8 {
        return self.flags >> 1 & 1;
    }

    pub fn interrupt(&self) -> u8 {
        return self.flags >> 2 & 1;
    }

    pub fn decimal(&self) -> u8 {
        return self.flags >> 3 & 1;
    }

    pub fn overflow(&self) -> u8 {
        return self.flags >> 6 & 1;
    }

    pub fn negative(&self) -> u8 {
        return self.flags >> 7 & 1;
    }

    pub fn set_carry(&mut self) {
        self.flags = self.flags | 0b0000_0001;
    }

    pub fn set_zero(&mut self) {
        self.flags = self.flags | 0b0000_0010;
    }

    pub fn set_interrupt(&mut self) {
        self.flags = self.flags | 0b0000_0100;
    }

    pub fn set_decimal(&mut self) {
        self.flags = self.flags | 0b0000_1000;
    }

    pub fn set_overflow(&mut self) {
        self.flags = self.flags | 0b0100_0000;
    }

    pub fn set_negative(&mut self) {
        self.flags = self.flags | 0b1000_0000;
    }

    pub fn clear_carry(&mut self) {
        self.flags = self.flags & 0b1111_1110;
    }

    pub fn clear_zero(&mut self) {
        self.flags = self.flags & 0b1111_1101;
    }

    pub fn clear_interrupt(&mut self) {
        self.flags = self.flags & 0b1111_1011;
    }

    pub fn clear_decimal(&mut self) {
        self.flags = self.flags & 0b1111_0111;
    }

    pub fn clear_overflow(&mut self) {
        self.flags = self.flags & 0b1011_1111;
    }

    pub fn clear_negative(&mut self) {
        self.flags = self.flags & 0b0111_1111;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_processor() {
        let processor = Processor::new();
        assert_eq!(processor.flags, 0b0011_0000);
    }

    #[test]
    fn test_set_carry() {
        let mut processor = Processor::new();
        assert_eq!(processor.carry(), 0);
        processor.set_carry();
        assert_eq!(processor.carry(), 1);
    }

    #[test]
    fn test_set_zero() {
        let mut processor = Processor::new();
        assert_eq!(processor.zero(), 0);
        processor.set_zero();
        assert_eq!(processor.zero(), 1);
    }

    #[test]
    fn test_set_interrupt() {
        let mut processor = Processor::new();
        assert_eq!(processor.interrupt(), 0);
        processor.set_interrupt();
        assert_eq!(processor.interrupt(), 1);
    }

    #[test]
    fn test_set_decimal() {
        let mut processor = Processor::new();
        assert_eq!(processor.decimal(), 0);
        processor.set_decimal();
        assert_eq!(processor.decimal(), 1);
    }

    #[test]
    fn test_set_overflow() {
        let mut processor = Processor::new();
        assert_eq!(processor.overflow(), 0);
        processor.set_overflow();
        assert_eq!(processor.overflow(), 1);
    }

    #[test]
    fn test_set_negative() {
        let mut processor = Processor::new();
        assert_eq!(processor.negative(), 0);
        processor.set_negative();
        assert_eq!(processor.negative(), 1);
    }

    #[test]
    fn test_clear_carry() {
        let mut processor = Processor::new();
        assert_eq!(processor.carry(), 0);
        processor.set_carry();
        assert_eq!(processor.carry(), 1);
        processor.clear_carry();
        assert_eq!(processor.carry(), 0);
    }

    #[test]
    fn test_clear_zero() {
        let mut processor = Processor::new();
        assert_eq!(processor.zero(), 0);
        processor.set_zero();
        assert_eq!(processor.zero(), 1);
        processor.clear_zero();
        assert_eq!(processor.zero(), 0);
    }

    #[test]
    fn test_clear_interrupt() {
        let mut processor = Processor::new();
        assert_eq!(processor.interrupt(), 0);
        processor.set_interrupt();
        assert_eq!(processor.interrupt(), 1);
        processor.clear_interrupt();
        assert_eq!(processor.interrupt(), 0);
    }

    #[test]
    fn test_clear_decimal() {
        let mut processor = Processor::new();
        assert_eq!(processor.decimal(), 0);
        processor.set_decimal();
        assert_eq!(processor.decimal(), 1);
        processor.clear_decimal();
        assert_eq!(processor.decimal(), 0);
    }

    #[test]
    fn test_clear_overflow() {
        let mut processor = Processor::new();
        assert_eq!(processor.overflow(), 0);
        processor.set_overflow();
        assert_eq!(processor.overflow(), 1);
        processor.clear_overflow();
        assert_eq!(processor.overflow(), 0);
    }

    #[test]
    fn test_clear_negative() {
        let mut processor = Processor::new();
        assert_eq!(processor.negative(), 0);
        processor.set_negative();
        assert_eq!(processor.negative(), 1);
        processor.clear_negative();
        assert_eq!(processor.negative(), 0);
    }

    #[test]
    fn test_clear_set_flags() {
        let mut processor = Processor::new();
        assert_eq!(processor.carry(), 0);
        assert_eq!(processor.zero(), 0);
        assert_eq!(processor.interrupt(), 0);
        assert_eq!(processor.decimal(), 0);
        assert_eq!(processor.overflow(), 0);
        assert_eq!(processor.negative(), 0);

        processor.set_carry();
        processor.set_zero();
        processor.set_interrupt();
        processor.set_decimal();
        processor.set_overflow();
        processor.set_negative();

        assert_eq!(processor.carry(), 1);
        assert_eq!(processor.zero(), 1);
        assert_eq!(processor.interrupt(), 1);
        assert_eq!(processor.decimal(), 1);
        assert_eq!(processor.overflow(), 1);
        assert_eq!(processor.negative(), 1);

        processor.clear_carry();
        processor.clear_zero();
        processor.clear_interrupt();
        processor.clear_decimal();
        processor.clear_overflow();
        processor.clear_negative();

        assert_eq!(processor.carry(), 0);
        assert_eq!(processor.zero(), 0);
        assert_eq!(processor.interrupt(), 0);
        assert_eq!(processor.decimal(), 0);
        assert_eq!(processor.overflow(), 0);
        assert_eq!(processor.negative(), 0);
    }
}
