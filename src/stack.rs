#[derive(Debug)]
pub struct Stack {
    bottom: u16,
    top: u16,
    _ptr: u8,
}

impl Stack {
    pub fn new(bottom: u16, top: u16) -> Self {
        let _ptr = bottom as u8;
        return Self { bottom, top, _ptr };
    }

    fn ptr(&self) -> u16 {
        return self._ptr.into();
    }

    pub fn decr_ptr(&mut self) {
        // stack grows downward
        self._ptr = self._ptr.wrapping_add(1);
    }

    pub fn incr_ptr(&mut self) {
        // stack grows downward
        self._ptr = self._ptr.wrapping_sub(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_stack() {
        let bottom: u16 = 0x01FF;
        let top: u16 = 0x0100;
        let s = Stack::new(bottom, top);
        assert_eq!(s.bottom, bottom);
        assert_eq!(s.top, top);
        assert_eq!(s._ptr, bottom as u8);
        assert_eq!(s.ptr(), (bottom as u8).into());
    }

    #[test]
    fn test_incr_ptr() {
        let bottom: u16 = 0x01FF;
        let top: u16 = 0x0100;
        let mut s = Stack::new(bottom, top);
        assert_eq!(s.ptr(), (bottom as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 1) as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 2) as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 3) as u8).into());
    }

    #[test]
    fn test_decr_ptr() {
        let bottom: u16 = 0x01FF;
        let top: u16 = 0x0100;
        let mut s = Stack::new(bottom, top);
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 5) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((bottom - 4) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((bottom - 3) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((bottom - 2) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((bottom - 1) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((bottom) as u8).into());
    }

    #[test]
    fn test_ptr_wrapping() {
        let bottom: u16 = 0x01FF;
        let top: u16 = 0x0100;
        let mut s = Stack::new(bottom, top);
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top + 1) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top + 2) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top + 3) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top + 4) as u8).into());
        s.decr_ptr();
        assert_eq!(s.ptr(), ((top + 5) as u8).into());

        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom) as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 1) as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 2) as u8).into());
        s.incr_ptr();
        assert_eq!(s.ptr(), ((bottom - 3) as u8).into());
    }
}
