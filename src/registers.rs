pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u32 {
    #[inline(always)]
    fn zero() -> Self {
        0
    }
}

impl Zero for u64 {
    #[inline(always)]
    fn zero() -> Self {
        0
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register {
    X1 = 0,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZeroOrRegister {
    Zero,
    Register(Register),
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Registers<T>([T; 31]);

const _: [(); 0] = [(); ((Register::X31 as usize + 1) * core::mem::size_of::<u32>())
    - core::mem::size_of::<Registers<u32>>()];

const _: [(); 0] = [(); ((Register::X31 as usize + 1) * core::mem::size_of::<u64>())
    - core::mem::size_of::<Registers<u64>>()];

impl<T: Copy + Default> Registers<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self([Default::default(); 31])
    }
}

impl<T: Copy> Registers<T> {
    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn get(&self, reg: Register) -> T {
        self.0[reg as usize]
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub fn get(&self, reg: Register) -> T {
        unsafe { *self.0.get_unchecked(reg as usize) }
    }
}

impl<T> Registers<T> {
    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn get_mut(&mut self, reg: Register) -> &mut T {
        &mut self.0[reg as usize]
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub fn get_mut(&mut self, reg: Register) -> &mut T {
        unsafe { self.0.get_unchecked_mut(reg as usize) }
    }
}

impl<T: Copy + Default> Default for Registers<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    #[inline(always)]
    pub fn fetch<T: Copy>(&self, regs: &Registers<T>) -> T {
        regs.get(*self)
    }

    #[inline(always)]
    pub fn fetch_mut<'a, T>(&self, regs: &'a mut Registers<T>) -> &'a mut T {
        regs.get_mut(*self)
    }
}

impl ZeroOrRegister {
    #[inline(always)]
    pub fn fetch<T: Copy + Zero>(&self, regs: &Registers<T>) -> T {
        match *self {
            Self::Zero => Zero::zero(),
            Self::Register(reg) => reg.fetch(regs),
        }
    }

    #[inline(always)]
    pub fn fetch_mut<'a, T>(&self, regs: &'a mut Registers<T>) -> Option<&'a mut T> {
        match *self {
            Self::Zero => None,
            Self::Register(reg) => Some(reg.fetch_mut(regs)),
        }
    }

    #[inline(always)]
    pub const unsafe fn decode_unchecked(raw: u8) -> Self {
        debug_assert!(raw < 32, "invalid register number");
        if raw == 0 {
            Self::Zero
        } else {
            Self::Register(core::mem::transmute(raw.wrapping_sub(1)))
        }
    }

    #[inline(always)]
    pub const fn decode_truncate(raw: u8) -> Self {
        unsafe { Self::decode_unchecked(raw & 0b11111) }
    }

    #[inline(always)]
    pub const fn decode(raw: u8) -> Option<Self> {
        if raw < 32 {
            Some(unsafe { Self::decode_unchecked(raw) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut regs = Registers::default();
        let x1 = regs.get_mut(Register::X1);
        *x1 = 20;
        let val = regs.get(Register::X1);
        assert_eq!(val, 20)
    }

    #[test]
    fn test_register() {
        let mut regs = Registers::default();
        let x1 = Register::X1;
        let x1_mut = x1.fetch_mut(&mut regs);
        *x1_mut = 20;
        assert_eq!(20, x1.fetch(&regs));
    }

    #[test]
    fn test_zero_or_register() {
        let mut regs = Registers::default();
        let reg = Register::X1;
        let x1 = reg.fetch_mut(&mut regs);
        *x1 = 20;
        match ZeroOrRegister::decode(1) {
            Some(ZeroOrRegister::Register(r)) => {
                assert_eq!(r, reg);
                assert_eq!(20, r.fetch(&regs));
            }
            _ => unreachable!(),
        };
        match ZeroOrRegister::decode(0) {
            Some(ZeroOrRegister::Zero) => {}
            _ => unreachable!(),
        }
    }
}
