use crate::decode::{U12, U5};
use crate::num::{As, Bitcast, Unsigned, UnsignedWrapping, Wrapping, Widening};

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

// M extension

pub trait Mul {
    fn mul(self, other: Self) -> Self;
}

pub trait Mulh {
    fn mulh(self, other: Self) -> Self;
}

pub trait Mulhsu {
    fn mulhsu(self, other: Self) -> Self;
}

pub trait Mulhu {
    fn mulhu(self, other: Self) -> Self;
}

pub trait Div {
    fn div(self, other: Self) -> Self;
}

pub trait Divu {
    fn divu(self, other: Self) -> Self;
}

pub trait Rem {
    fn rem(self, other: Self) -> Self;
}

pub trait Remu {
    fn remu(self, other: Self) -> Self;
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
    + Mul
    + Mulh
    + Mulhsu
    + Mulhu
    + Div
    + Divu
    + Rem
    + Remu
{
}

pub trait BaseMathW:
    BaseMath + Addw + Subw + Sllw + Srlw + Sraw + Addiw + Slliw + Srliw + Sraiw
{
}

impl<T: Wrapping> Add for T {
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        T::wrapping_add(self, other)
    }
}

impl<T: Wrapping> Sub for T {
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        T::wrapping_sub(self, other)
    }
}

impl<T> Sll for T
where
    T: Wrapping,
    T: As<u32>,
{
    #[inline(always)]
    fn sll(self, other: Self) -> Self {
        T::wrapping_shl(self, other.r#as())
    }
}

impl<T> Slt for T
where
    T: Unsigned,
    T: core::cmp::Ord,
    <T as Unsigned>::Signed: core::cmp::Ord,
    bool: As<T>,
{
    #[inline(always)]
    fn slt(self, other: Self) -> Self {
        (Bitcast::<<T as Unsigned>::Signed>::bitcast(self)
            < Bitcast::<<T as Unsigned>::Signed>::bitcast(other))
        .r#as()
    }
}

impl<T> Sltu for T
where
    T: core::cmp::Ord,
    bool: As<T>,
{
    #[inline(always)]
    fn sltu(self, other: Self) -> Self {
        (self < other).r#as()
    }
}

impl<T: core::ops::BitXor<Output = T>> Xor for T {
    #[inline(always)]
    fn xor(self, other: Self) -> Self {
        core::ops::BitXor::bitxor(self, other)
    }
}

impl<T> Srl for T
where
    T: Wrapping,
    T: As<u32>,
{
    #[inline(always)]
    fn srl(self, other: Self) -> Self {
        T::wrapping_shr(self, other.r#as())
    }
}

impl<T> Sra for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: Wrapping,
    T: As<u32>,
{
    #[inline(always)]
    fn sra(self, other: Self) -> Self {
        (<T as Bitcast<T::Signed>>::bitcast(self))
            .wrapping_shr(other.r#as())
            .bitcast()
    }
}

impl<T: core::ops::BitOr<Output = T>> Or for T {
    #[inline(always)]
    fn or(self, other: Self) -> Self {
        core::ops::BitOr::bitor(self, other)
    }
}

impl<T: core::ops::BitAnd<Output = T>> And for T {
    #[inline(always)]
    fn and(self, other: Self) -> Self {
        core::ops::BitAnd::bitand(self, other)
    }
}

impl<T> Addi for T
where
    T: UnsignedWrapping,
    i16: As<<T as Unsigned>::Signed>,
{
    #[inline(always)]
    fn addi(self, other: U12) -> Self {
        T::wrapping_add_signed(self, other.sign_extend().r#as())
    }
}

impl<T> Slti for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: core::cmp::Ord,
    i16: As<T>,
    bool: As<T>,
{
    #[inline(always)]
    fn slti(self, other: U12) -> Self {
        (Bitcast::<<T as Unsigned>::Signed>::bitcast(self)
            < Bitcast::<<T as Unsigned>::Signed>::bitcast(other.sign_extend().r#as()))
        .r#as()
    }
}

impl<T: Copy + Unsigned> Sltiu for T
where
    T: core::cmp::Ord,
    u16: As<T>,
    bool: As<T>,
{
    #[inline(always)]
    fn sltiu(self, other: U12) -> Self {
        (self < other.as_u16().r#as()).r#as()
    }
}

impl<T> Xori for T
where
    T: core::ops::BitXor<Output = T>,
    u16: As<T>,
{
    #[inline(always)]
    fn xori(self, other: U12) -> Self {
        core::ops::BitXor::bitxor(self, other.as_u16().r#as())
    }
}

impl<T> Ori for T
where
    T: core::ops::BitOr<Output = T>,
    u16: As<T>,
{
    #[inline(always)]
    fn ori(self, other: U12) -> Self {
        core::ops::BitOr::bitor(self, other.as_u16().r#as())
    }
}

impl<T> Andi for T
where
    T: core::ops::BitAnd<Output = T>,
    u16: As<T>,
{
    #[inline(always)]
    fn andi(self, other: U12) -> Self {
        core::ops::BitAnd::bitand(self, other.as_u16().r#as())
    }
}

impl<T: Wrapping> Slli for T {
    #[inline(always)]
    fn slli(self, other: U5) -> Self {
        self.wrapping_shl(other.as_u32())
    }
}

impl<T: Wrapping> Srli for T {
    #[inline(always)]
    fn srli(self, other: U5) -> Self {
        self.wrapping_shr(other.as_u32())
    }
}

impl<T> Srai for T
where
    T: Unsigned,
    T: Wrapping,
    <T as Unsigned>::Signed: Wrapping,
{
    #[inline(always)]
    fn srai(self, other: U5) -> Self {
        (<T as Bitcast<T::Signed>>::bitcast(self))
            .wrapping_shr(other.as_u32())
            .bitcast()
    }
}

impl<T: Widening> Mul for T {
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        T::widening_mul(self, other).0
    }
}

impl<T: Widening> Mulh for T {
    #[inline(always)]
    fn mulh(self, other: Self) -> Self {
        T::widening_mul(self, other).1
    }
}

impl<T: Widening> Mulhu for T {
    #[inline(always)]
    fn mulhu(self, other: Self) -> Self {
        T::widening_mul(self, other).1
    }
}

impl<T: Widening> Mulhsu for T {
    #[inline(always)]
    fn mulhsu(self, other: Self) -> Self {
        T::widening_mul(self, other).1
    }
}

impl<T: Wrapping> Div for T {
    #[inline(always)]
    fn div(self, other: Self) -> Self {
        T::wrapping_div(self, other)
    }
}

impl<T: Wrapping> Divu for T {
    #[inline(always)]
    fn divu(self, other: Self) -> Self {
        T::wrapping_div(self, other)
    }
}

impl<T: Wrapping> Rem for T {
    #[inline(always)]
    fn rem(self, other: Self) -> Self {
        T::wrapping_rem(self, other)
    }
}

impl<T: Wrapping> Remu for T {
    #[inline(always)]
    fn remu(self, other: Self) -> Self {
        T::wrapping_rem(self, other)
    }
}

impl<
        T: Copy
            + Add
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
            + Mul
            + Mulh
            + Mulhsu
            + Mulhu
            + Div
            + Divu
            + Rem
            + Remu
    > BaseMath for T
{
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

impl BaseMathW for u64 {}
