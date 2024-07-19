const B12_MASK: u32 = bitmask(12);
const B7_MASK: u32 = bitmask(7);
const B6_MASK: u32 = bitmask(6);
const B5_MASK: u32 = bitmask(5);
const B4_MASK: u32 = bitmask(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct R {
    pub funct7: U7,
    pub rs2: U5,
    pub rs1: U5,
    pub funct3: U3,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct R4 {
    pub rs3: U5,
    pub funct2: U2,
    pub rs2: U5,
    pub rs1: U5,
    pub funct3: U3,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I {
    pub imm: U12,
    pub rs1: U5,
    pub funct3: U3,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shift {
    pub prefix: U7,
    pub shamt: U5,
    pub rs1: U5,
    pub funct3: U3,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fence {
    pub fm: U4,
    pub pred: U4,
    pub succ: U4,
    pub rs1: U5,
    pub funct3: U3,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S {
    pub imm: U12,
    pub rs2: U5,
    pub rs1: U5,
    pub funct3: U3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct B {
    pub imm: U13,
    pub rs2: U5,
    pub rs1: U5,
    pub funct3: U3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U {
    pub imm: u32,
    pub rd: U5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J {
    pub imm: U21,
    pub rd: U5,
}

impl R {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            funct7: unsafe { U7::new_unchecked((value >> 25) as u8) },
            rs2: U5::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }

    #[inline(always)]
    pub const fn id(&self) -> U10 {
        unsafe { U10::new_unchecked((self.funct7.as_u16() << 3) | self.funct3.as_u16()) }
    }

    #[inline(always)]
    pub fn fid(&self) -> U12 {
        let need_f3 = [0b0010000u16, 0b0010100, 0b1110000, 0b1010000, 0b1111000];
        let need_r2 = [0b1100000u16, 0b1101000];
        unsafe {
            U12::new_unchecked(if need_f3.contains(&self.funct7.as_u16()) {
                self.funct7.as_u16() << 3 | self.funct3.as_u16()
            } else if need_r2.contains(&self.funct7.as_u16()) {
                self.funct7.as_u16() << 5 | self.rs2.as_u16()
            } else {
                self.funct7.as_u16()
            })
        }
    }
}

impl From<u32> for R {
    #[inline(always)]
    fn from(value: u32) -> Self {
        R::from_u32(value)
    }
}

impl R4 {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            rs3: unsafe { U5::new_unchecked((value >> 27) as u8) },
            funct2: U2::new_truncate((value >> 25) as u8),
            rs2: U5::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }
}

impl From<u32> for R4 {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl I {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            imm: unsafe { U12::new_unchecked((value >> 20) as u16) },
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }

    #[inline(always)]
    pub const fn id(&self) -> U3 {
        self.funct3
    }
}

impl From<u32> for I {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl Shift {
    #[inline(always)]
    pub const fn from_i(
        I {
            imm,
            rs1,
            funct3,
            rd,
        }: I,
    ) -> Self {
        Self {
            prefix: unsafe { U7::new_unchecked((imm.as_u16() >> 5) as u8) },
            shamt: U5::new_truncate(imm.as_u16() as u8),
            rs1,
            funct3,
            rd,
        }
    }

    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self::from_i(I::from_u32(value))
    }

    #[inline(always)]
    pub const fn id(&self) -> U10 {
        unsafe { U10::new_unchecked((self.prefix.as_u16() << 3) | self.funct3.as_u16()) }
    }
}

impl From<I> for Shift {
    #[inline(always)]
    fn from(value: I) -> Self {
        Self::from_i(value)
    }
}

impl From<u32> for Shift {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl Fence {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            fm: unsafe { U4::new_unchecked((value >> 28) as u8) },
            pred: U4::new_truncate((value >> 24) as u8),
            succ: U4::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 16) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }
}

impl From<u32> for Fence {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl S {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            imm: unsafe {
                U12::new_unchecked(
                    ((((value >> 25) & B7_MASK) << 5) | ((value >> 7) & B5_MASK)) as u16,
                )
            },
            rs2: U5::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
        }
    }

    #[inline(always)]
    pub const fn id(&self) -> U3 {
        self.funct3
    }
}

impl From<u32> for S {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl B {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            imm: unsafe {
                U13::new_unchecked(
                    (
                        // 12
                        (value >> 19) & (1 << 12)
                        // 11
                        | (value << 4) & (1 << 11)
                        // 10:5
                        | (value >> 20) & (B6_MASK << 5)
                        // 4:1
                        | (value >> 7) & (B4_MASK << 1)
                    ) as u16,
                )
            },
            rs2: U5::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
        }
    }

    #[inline(always)]
    pub const fn id(&self) -> U3 {
        self.funct3
    }
}

impl From<u32> for B {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl U {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            imm: value & !B12_MASK,
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }
}

impl From<u32> for U {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl J {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self {
            imm: unsafe {
                U21::new_unchecked(
                    // 20
                    ((value & (1 << 31)) >> 11)
                    // 10:1
                    | ((value & (0b1111111111 << 21)) >> 20)
                    // 11
                    | ((value & (1 << 20)) >> 9)
                    // 19:12
                    | (value & (0b11111111 << 12)),
                )
            },
            rd: U5::new_truncate((value >> 7) as u8),
        }
    }
}

impl From<u32> for J {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

mod __sealed {
    pub trait Unsigned {
        type Signed;
    }

    impl Unsigned for u8 {
        type Signed = i8;
    }

    impl Unsigned for u16 {
        type Signed = i16;
    }

    impl Unsigned for u32 {
        type Signed = i32;
    }
}

macro_rules! impl_base {
    (@def $t:ident, $base:ty) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $t($base);
    };

    (@minmax $base:ty, $bits:expr) => {
        pub const BITS: u32 = $bits;
        pub const MIN: Self = Self(0);
        pub const BITMASK: $base = (1 << Self::BITS) - 1;
        pub const MAX: Self = Self(Self::BITMASK);
    };

    (@new $base:ty) => {
        #[inline(always)]
        pub const fn new(value: $base) -> Option<Self> {
            if value > Self::MAX.0 {
                None
            } else {
                Some(Self(value))
            }
        }

        #[inline(always)]
        pub const unsafe fn new_unchecked(value: $base) -> Self {
            debug_assert!(value <= Self::MAX.0, concat!(stringify!($base), " too large"));
            Self(value)
        }

        #[inline(always)]
        pub const fn new_truncate(value: $base) -> Self {
            Self(value & Self::BITMASK)
        }
    };

    (@sign $t:ident, $base:ty) => {
        #[inline(always)]
        pub const fn sign_extend(&self) -> <$base as __sealed::Unsigned>::Signed {
            const OTHER_BITS: u32 = <$base as __sealed::Unsigned>::Signed::BITS - <$t>::BITS;
            unsafe { core::mem::transmute::<$base, <$base as __sealed::Unsigned>::Signed>(self.0) }
                .wrapping_shl(OTHER_BITS).wrapping_shr(OTHER_BITS)
        }
    };

    (@const_from_internal $base:ty $(,)?) => {};
    (@const_from_internal $base:ty, $dt:ty => $dname:ident $(,)?) => {
        #[inline(always)]
        pub const fn $dname(value: $dt) -> Self {
            Self(value as $base)
        }
    };
    (@const_from_internal $base:ty, $dt:ty => $dname:ident, $($tt:tt)+) => {
        impl_base!(@const_from_internal $base, $dt => $dname);
        impl_base!(@const_from_internal $base, $($tt)+);
    };
    (@const_from_internal $base:ty, $dt:ty > $dmeth:ident => $dname:ident $(,)?) => {
        #[inline(always)]
        pub const fn $dname(value: $dt) -> Self {
            Self(value.$dmeth() as $base)
        }
    };
    (@const_from_internal $base:ty, $dt:ty > $dmeth:ident => $dname:ident, $($tt:tt)+) => {
        impl_base!(@const_from_internal $base, $dt > $dmeth => $dname);
        impl_base!(@const_from_internal $base, $($tt)+);
    };
    (@const_from $base:ty $(,)*) => {};
    (@const_from $base:ty, $($tt:tt)+) => {
        impl_base!(@const_from_internal $base, $($tt)+);
    };

    (@const_into_internal $(,)?) => {};
    (@const_into_internal $t:ty => $dname:ident $(,)?) => {
        #[inline(always)]
        pub const fn $dname(&self) -> $t {
            self.0 as $t
        }
    };
    (@const_into_internal $t:ty => $dname:ident, $($tt:tt)+) => {
        impl_base!(@const_into_internal $t => $dname);
        impl_base!(@const_into_internal $($tt)+);
    };
    (@const_into $($tt:tt)*) => {
        impl_base!(@const_into_internal $($tt)*);
    };

    (@from_internal $t:ident, $base:ty $(,)?) => {};
    (@from_internal $t:ident, $base:ty, $dt:ty => $dname:ident $(,)?) => {
        impl From<$dt> for $t {
            #[inline(always)]
            fn from(value: $dt) -> Self {
                Self::$dname(value)
            }
        }
    };
    (@from_internal $t:ident, $base:ty, $dt:ty => $dname:ident, $($tt:tt)+) => {
        impl_base!(@from_internal $t, $base, $dt => $dname);
        impl_base!(@from_internal $t, $base, $($tt)+);
    };
    (@from_internal $t:ident, $base:ty, $dt:ty > $dmeth:ident => $dname:ident $(,)?) => {
        impl_base!(@from_internal $t, $base, $dt => $dname);
    };
    (@from_internal $t:ident, $base:ty, $dt:ty > $dmeth:ident => $dname:ident, $($tt:tt)+) => {
        impl_base!(@from_internal $t, $base, $dt > $dmeth => $dname);
        impl_base!(@from_internal $t, $base, $($tt)+);
    };
    (@from $t:ident, $base:ty $(,)*) => {};
    (@from $t:ident, $base:ty, $($tt:tt)+) => {
        impl_base!(@from_internal $t, $base, $($tt)+);
    };

    (@into_internal $t:ident $(,)?) => {};
    (@into_internal $t:ident, $dt:ty => $dname:ident $(,)?) => {
        impl From<$t> for $dt {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.$dname()
            }
        }
    };
    (@into_internal $t:ident, $base:ty => $dname:ident, $($tt:tt)+) => {
        impl_base!(@into_internal $t, $base => $dname);
        impl_base!(@into_internal $t, $($tt)+);
    };
    (@into $t:ident $(,)*) => {};
    (@into $t:ident, $($tt:tt)+) => {
        impl_base!(@into_internal $t, $($tt)+);
    };
}

macro_rules! impl_u8 {
    ($t:ident, $bits:expr) => {
        impl_u8!($t, $bits,);
    };
    ($t:ident, $bits:expr, $($tt:tt)*) => {
        impl_base!(@def $t, u8);

        impl $t {
            impl_base!(@minmax u8, $bits);

            impl_base!(@new u8);

            impl_base!(@sign $t, u8);

            impl_base!(@const_from u8, $($tt)*);

            impl_base!(@const_into u8  => as_u8,
                                   u16 => as_u16,
                                   u32 => as_u32,
                                   u64 => as_u64);
        }

        impl_base!(@from $t, u8, $($tt)*);
        impl_base!(@into $t, u8  => as_u8,
                             u16 => as_u16,
                             u32 => as_u32,
                             u64 => as_u64);
    };
}

macro_rules! impl_u16 {
    ($t:ident, $bits:expr) => {
        impl_u16!($t, $bits,);
    };
    ($t:ident, $bits:expr, $($tt:tt)*) => {
        impl_base!(@def $t, u16);

        impl $t {
            impl_base!(@minmax u16, $bits);

            impl_base!(@new u16);

            impl_base!(@sign $t, u16);

            impl_base!(@const_from u16,
                       u8 => from_u8,
                       $($tt)*);

            impl_base!(@const_into u16 => as_u16,
                                   u32 => as_u32,
                                   u64 => as_u64);
        }

        impl_base!(@from $t, u16,
                   u8 => from_u8,
                   $($tt)*);

        impl_base!(@into $t, u16 => as_u16,
                             u32 => as_u32,
                             u64 => as_u64);
    };
}

macro_rules! impl_u32 {
    ($t:ident, $bits:expr) => {
        impl_u32!($t, $bits,);
    };
    ($t:ident, $bits:expr, $($tt:tt)*) => {
        impl_base!(@def $t, u32);

        impl $t {
            impl_base!(@minmax u32, $bits);

            impl_base!(@new u32);

            impl_base!(@sign $t, u32);

            impl_base!(@const_from u32,
                       u8 => from_u8,
                       u16 => from_u16,
                       $($tt)*);

            impl_base!(@const_into u32 => as_u32,
                                   u64 => as_u64);
        }

        impl_base!(@from $t, u32,
                   u8 => from_u8,
                   u16 => from_u16,
                   $($tt)*);

        impl_base!(@into $t, u32 => as_u32,
                             u64 => as_u64);
    };
}

impl_u8!(U2, 2);
impl_u8!( U3, 3,
          U2  > as_u8  => from_u2);
impl_u8!( U4, 4,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3);
impl_u8!( U5, 5,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4);
impl_u8!( U7, 7,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5);
impl_u16!(U10, 10,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7);
impl_u16!(U12, 12,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7,
          U10 > as_u16 => from_u10);
impl_u16!(U13, 13,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7,
          U10 > as_u16 => from_u10,
          U12 > as_u16 => from_u12);
impl_u32!(U21, 21,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7,
          U10 > as_u16 => from_u10,
          U12 > as_u16 => from_u12,
          U13 > as_u16 => from_u13);

#[inline(always)]
const fn bitmask(bits: u32) -> u32 {
    (1 << bits) - 1
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn sign_extend() {
        assert_eq!(
            U13::new_truncate(0b1111111111110u16).sign_extend(),
            unsafe { core::mem::transmute::<u16, i16>(0b1111111111111110u16) }
        );
        assert_eq!(
            U13::new_truncate(0b0111111111110u16).sign_extend(),
            unsafe { core::mem::transmute::<u16, i16>(0b0000111111111110u16) }
        );
    }

    #[test]
    fn decode_r() {
        assert_eq!(
            R::from(0b0000000_00001_00010_000_00100_0110011),
            R {
                rd: U5::new_truncate(4),
                funct3: U3::new_truncate(0),
                rs1: U5::new_truncate(2),
                rs2: U5::new_truncate(1),
                funct7: U7::new_truncate(0)
            }
        );
    }

    #[test]
    fn decode_i() {
        assert_eq!(
            I::from(0b000000000000_00001_000_00010_0010011),
            I {
                rd: U5::new_truncate(2),
                funct3: U3::new_truncate(0),
                rs1: U5::new_truncate(1),
                imm: U12::new_truncate(0),
            }
        );
    }

    #[test]
    fn decode_shift() {
        assert_eq!(
            Shift::from(0b000000000011_00001_000_00010_0010011),
            Shift {
                rd: U5::new_truncate(2),
                funct3: U3::new_truncate(0),
                rs1: U5::new_truncate(1),
                prefix: U7::new_truncate(0),
                shamt: U5::new_truncate(3),
            }
        );
    }

    #[test]
    fn decode_s() {
        assert_eq!(
            S::from(0b0000000_00001_00010_010_00100_0100011),
            S {
                funct3: U3::new_truncate(2),
                imm: U12::new_truncate(4),
                rs1: U5::new_truncate(2),
                rs2: U5::new_truncate(1),
            }
        );
        assert_eq!(
            S::from(0b0000000_00001_00010_011_11000_0100011),
            S {
                funct3: U3::new_truncate(3),
                imm: U12::new_truncate(24),
                rs1: U5::new_truncate(2),
                rs2: U5::new_truncate(1),
            }
        );
    }

    #[test]
    fn decode_b() {
        assert_eq!(
            B::from(0b1_000000_00000_00001_000_0000_0_0000000u32),
            B {
                imm: U13::new_truncate(0b1_0_000000_0000_0),
                rs2: U5::new_truncate(0),
                rs1: U5::new_truncate(1),
                funct3: U3::new_truncate(0),
            }
        );

        assert_eq!(
            B::from(0b0_111111_00010_00011_000_0000_0_0000000u32),
            B {
                imm: U13::new_truncate(0b0_0_111111_0000_0),
                rs2: U5::new_truncate(2),
                rs1: U5::new_truncate(3),
                funct3: U3::new_truncate(0),
            }
        );

        assert_eq!(
            B::from(0b0_000000_00100_00101_000_1111_0_0000000u32),
            B {
                imm: U13::new_truncate(0b0_0_000000_1111_0),
                rs2: U5::new_truncate(4),
                rs1: U5::new_truncate(5),
                funct3: U3::new_truncate(0),
            }
        );

        assert_eq!(
            B::from(0b0_000000_00110_00111_000_0000_1_0000000u32),
            B {
                imm: U13::new_truncate(0b0_1_000000_0000_0),
                rs2: U5::new_truncate(6),
                rs1: U5::new_truncate(7),
                funct3: U3::new_truncate(0),
            }
        );
    }

    #[test]
    fn decode_u() {
        assert_eq!(
            U::from(0b00000000000000011000_00010_0010111),
            U {
                rd: U5::new_truncate(2),
                imm: 24u32 << 12,
            }
        );
    }

    #[test]
    fn decode_j() {
        assert_eq!(
            J::from((0b10000000000100000000 << 12) | (20 << 7)),
            J {
                imm: U21::new_truncate(1 << 20 | 1 << 11),
                rd: U5::new_truncate(20),
            }
        );

        assert_eq!(
            J::from(0b01111111111011111111 << 12),
            J {
                imm: U21::new_truncate(!(1 << 20 | 1 << 11 | 1)),
                rd: U5::new_truncate(0),
            },
        );
    }
}
