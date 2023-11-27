use crate::decode::{U12, U5};

pub trait Add {
    fn add(self, other: Self) -> Self;
}

pub trait Sub {
    fn sub(self, other: Self) -> Self;
}

pub trait Sll {
    fn sll(self, other: Self) -> Self;
}

pub trait Slt {
    fn slt(self, other: Self) -> Self;
}

pub trait Sltu {
    fn sltu(self, other: Self) -> Self;
}

pub trait Xor {
    fn xor(self, other: Self) -> Self;
}

pub trait Srl {
    fn srl(self, other: Self) -> Self;
}

pub trait Sra {
    fn sra(self, other: Self) -> Self;
}

pub trait Or {
    fn or(self, other: Self) -> Self;
}

pub trait And {
    fn and(self, other: Self) -> Self;
}

pub trait Addi {
    fn addi(self, other: U12) -> Self;
}

pub trait Slti {
    fn slti(self, other: U12) -> Self;
}

pub trait Sltiu {
    fn sltiu(self, other: U12) -> Self;
}

pub trait Xori {
    fn xori(self, other: U12) -> Self;
}

pub trait Ori {
    fn ori(self, other: U12) -> Self;
}

pub trait Andi {
    fn andi(self, other: U12) -> Self;
}

pub trait Slli {
    fn slli(self, other: U5) -> Self;
}

pub trait Srli {
    fn srli(self, other: U5) -> Self;
}

pub trait Srai {
    fn srai(self, other: U5) -> Self;
}

pub trait Addw {
    fn addw(self, other: Self) -> Self;
}

pub trait Subw {
    fn subw(self, other: Self) -> Self;
}

pub trait Sllw {
    fn sllw(self, other: Self) -> Self;
}

pub trait Srlw {
    fn srlw(self, other: Self) -> Self;
}

pub trait Sraw {
    fn sraw(self, other: Self) -> Self;
}

pub trait Addiw {
    fn addiw(self, other: U12) -> Self;
}

pub trait Slliw {
    fn slliw(self, other: Self) -> Self;
}

pub trait Srliw {
    fn srliw(self, other: Self) -> Self;
}

pub trait Sraiw {
    fn sraiw(self, other: Self) -> Self;
}

pub trait BaseMath:
    Add
    + Sub
    + Sll
    + Slt
    + Sltu
    + Xor
    + Srl
    + Sra
    + Or
    + And
    + Addi
    + Slti
    + Sltiu
    + Xori
    + Ori
    + Andi
    + Slli
    + Srli
    + Srai
{
}

pub trait BaseMathW:
    BaseMath + Addw + Subw + Sllw + Srlw + Sraw + Addiw + Slliw + Srliw + Sraiw
{
}

macro_rules! impl_math {
    (BASE = $t:ty, SIGNED = $st:ty, SHIFT = $bt:ty, CAST_METHOD = $cast_method:ident $(,)?) => {
        impl Add for $t {
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                <$t>::wrapping_add(self, other)
            }
        }

        impl Sub for $t {
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                <$t>::wrapping_sub(self, other)
            }
        }

        impl Sll for $t {
            #[inline(always)]
            fn sll(self, other: Self) -> Self {
                <$t>::wrapping_shl(self, other as $bt)
            }
        }

        impl Slt for $t {
            #[inline(always)]
            fn slt(self, other: Self) -> Self {
                unsafe {
                    (core::mem::transmute::<_, $st>(self) < core::mem::transmute(other)) as $t
                }
            }
        }

        impl Sltu for $t {
            #[inline(always)]
            fn sltu(self, other: Self) -> Self {
                (self < other) as Self
            }
        }

        impl Xor for $t {
            #[inline(always)]
            fn xor(self, other: Self) -> Self {
                std::ops::BitXor::bitxor(self, other)
            }
        }

        impl Srl for $t {
            #[inline(always)]
            fn srl(self, other: Self) -> Self {
                <$t>::wrapping_shr(self, other as $bt)
            }
        }

        impl Sra for $t {
            #[inline(always)]
            fn sra(self, other: Self) -> Self {
                unsafe {
                    core::mem::transmute(
                        core::mem::transmute::<_, $st>(self).wrapping_shr(other as $bt),
                    )
                }
            }
        }

        impl Or for $t {
            #[inline(always)]
            fn or(self, other: Self) -> Self {
                std::ops::BitOr::bitor(self, other)
            }
        }

        impl And for $t {
            #[inline(always)]
            fn and(self, other: Self) -> Self {
                std::ops::BitAnd::bitand(self, other)
            }
        }

        impl Addi for $t {
            #[inline(always)]
            fn addi(self, other: U12) -> Self {
                <$t>::wrapping_add_signed(self, other.sign_extend() as $st)
            }
        }

        impl Slti for $t {
            #[inline(always)]
            fn slti(self, other: U12) -> Self {
                Slt::slt(self, other.sign_extend() as $t)
            }
        }

        impl Sltiu for $t {
            #[inline(always)]
            fn sltiu(self, other: U12) -> Self {
                Sltu::sltu(self, other.$cast_method())
            }
        }

        impl Xori for $t {
            #[inline(always)]
            fn xori(self, other: U12) -> Self {
                Xor::xor(self, other.$cast_method())
            }
        }

        impl Ori for $t {
            #[inline(always)]
            fn ori(self, other: U12) -> Self {
                Or::or(self, other.$cast_method())
            }
        }

        impl Andi for $t {
            #[inline(always)]
            fn andi(self, other: U12) -> Self {
                And::and(self, other.$cast_method())
            }
        }

        impl Slli for $t {
            #[inline(always)]
            fn slli(self, other: U5) -> Self {
                Sll::sll(self, other.$cast_method())
            }
        }

        impl Srli for $t {
            #[inline(always)]
            fn srli(self, other: U5) -> Self {
                Srl::srl(self, other.$cast_method())
            }
        }

        impl Srai for $t {
            #[inline(always)]
            fn srai(self, other: U5) -> Self {
                Sra::sra(self, other.$cast_method())
            }
        }

        impl BaseMath for $t {}
    };
}

impl_math! {
    BASE = u32,
    SIGNED = i32,
    SHIFT = u32,
    CAST_METHOD = as_u32,
}

impl_math! {
    BASE = u64,
    SIGNED = i64,
    SHIFT = u32,
    CAST_METHOD = as_u64,
}

impl Addw for u64 {
    #[inline(always)]
    fn addw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_add(other as u32),
            ) as i64)
        }
    }
}

impl Subw for u64 {
    #[inline(always)]
    fn subw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_sub(other as u32),
            ) as i64)
        }
    }
}

impl Sllw for u64 {
    #[inline(always)]
    fn sllw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_shl(other as u32),
            ) as i64)
        }
    }
}

impl Srlw for u64 {
    #[inline(always)]
    fn srlw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_shr(other as u32),
            ) as i64)
        }
    }
}

impl Sraw for u64 {
    #[inline(always)]
    fn sraw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(
                core::mem::transmute::<_, i32>(self as u32).wrapping_shr(other as u32) as i64,
            )
        }
    }
}

impl Addiw for u64 {
    #[inline(always)]
    fn addiw(self, other: U12) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_add_signed(other.sign_extend() as i32),
            ) as i64)
        }
    }
}

impl Slliw for u64 {
    #[inline(always)]
    fn slliw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_shl(other as u32),
            ) as i64)
        }
    }
}

impl Srliw for u64 {
    #[inline(always)]
    fn srliw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(
                (self as u32).wrapping_shr(other as u32),
            ) as i64)
        }
    }
}

impl Sraiw for u64 {
    #[inline(always)]
    fn sraiw(self, other: Self) -> Self {
        unsafe {
            core::mem::transmute(
                core::mem::transmute::<_, i32>(self as u32).wrapping_shr(other as u32) as i64,
            )
        }
    }
}
