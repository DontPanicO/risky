use std::cmp::Ordering;
use std::num::FpCategory;

use crate::decode::{U12, U5};
use crate::num::{As, Bitcast, Shiftable, Unsigned, UnsignedWrapping, Wrapping};

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

pub trait Fadd {
    fn fadd(self, other: Self) -> Self;
}

pub trait Fsub {
    fn fsub(self, other: Self) -> Self;
}

pub trait Fmul {
    fn fmul(self, other: Self) -> Self;
}

pub trait Fdiv {
    fn fdiv(self, other: Self) -> Self;
}

pub trait Fsqrt {
    fn fsqrt(self, other: Self) -> Self;
}

pub trait Fsgnj {
    fn fsgnj(self, other: Self) -> Self;
}

pub trait Fsgnjn {
    fn fsgnjn(self, other: Self) -> Self;
}

pub trait Fsgnjx {
    fn fsgnjx(self, other: Self) -> Self;
}

pub trait Fmin {
    fn fmin(self, other: Self) -> Self;
}

pub trait Fmax {
    fn fmax(self, other: Self) -> Self;
}

pub trait Fcvtws {
    fn fcvtws(self, other: Self) -> Self;
}

pub trait Fcvtwus {
    fn fcvtwus(self, other: Self) -> Self;
}

pub trait Fmvxw {
    fn fmvxw(self, other: Self) -> Self;
}

pub trait Feq {
    fn feq(self, other: Self) -> Self;
}

pub trait Flt {
    fn flt(self, other: Self) -> Self;
}

pub trait Fle {
    fn fle(self, other: Self) -> Self;
}

pub trait Fclass {
    fn fclass(self, other: Self) -> Self;
}

pub trait Fcvtsw {
    fn fcvtsw(self, other: Self) -> Self;
}

pub trait Fcvtswu {
    fn fcvtswu(self, other: Self) -> Self;
}

pub trait Fmvwx {
    fn fmvwx(self, other: Self) -> Self;
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

pub trait BaseFloat:
    Copy
    + Fadd
    + Fsub
    + Fmul
    + Fdiv
    + Fsqrt
    + Fsgnj
    + Fsgnjn
    + Fsgnjx
    + Fmin
    + Fmax
    + Fcvtws
    + Fcvtwus
    + Fmvxw
    + Feq
    + Fle
    + Flt
    + Fclass
    + Fcvtsw
    + Fcvtswu
    + Fmvwx
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

impl<T: Wrapping> Mul for T {
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        T::wrapping_mul(self, other)
    }
}

impl<T> Mulh for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: Shiftable,
    <<T as Unsigned>::Signed as Shiftable>::To: Wrapping,
    <T as Unsigned>::Signed: As<<<T as Unsigned>::Signed as Shiftable>::To>,
    <<T as Unsigned>::Signed as Shiftable>::To: As<T>,
{
    #[inline(always)]
    fn mulh(self, other: Self) -> Self {
        <T as Bitcast<T::Signed>>::bitcast(self)
            .r#as()
            .wrapping_mul(<T as Bitcast<T::Signed>>::bitcast(other).r#as())
            .wrapping_shr(<<T as Unsigned>::Signed as Shiftable>::SHIFT_BITS)
            .r#as()
    }
}

impl<T> Mulhu for T
where
    T: Unsigned,
    T: Wrapping,
    T: Shiftable,
    <T as Shiftable>::To: Wrapping,
    <T as Shiftable>::To: As<T>,
    T: As<<T as Shiftable>::To>,
{
    #[inline(always)]
    fn mulhu(self, other: Self) -> Self {
        <T as Shiftable>::To::wrapping_mul(self.r#as(), other.r#as())
            .wrapping_shr(<T as Shiftable>::SHIFT_BITS)
            .r#as()
    }
}

impl<T: Wrapping> Mulhsu for T {
    #[inline(always)]
    fn mulhsu(self, other: Self) -> Self {
        // TODO: implement it
        T::wrapping_add(self, other)
    }
}

impl<T> Div for T
where
    T: Unsigned,
    T: Wrapping,
    <T as Unsigned>::Signed: Wrapping,
    <T as Unsigned>::Signed: As<T>,
{
    #[inline(always)]
    fn div(self, other: Self) -> Self {
        self.r#as().wrapping_div(other.r#as()).r#as()
    }
}

impl<T: Wrapping> Divu for T {
    #[inline(always)]
    fn divu(self, other: Self) -> Self {
        T::wrapping_div(self, other)
    }
}

impl<T> Rem for T
where
    T: Unsigned,
    T: Wrapping,
    <T as Unsigned>::Signed: Wrapping,
    <T as Unsigned>::Signed: As<T>,
{
    #[inline(always)]
    fn rem(self, other: Self) -> Self {
        self.r#as().wrapping_rem(other.r#as()).r#as()
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
            + Remu,
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

impl Fadd for u32 {
    #[inline(always)]
    fn fadd(self, other: Self) -> Self {
        (f32::from_bits(self) + f32::from_bits(other)).to_bits()
    }
}

impl Fsub for u32 {
    #[inline(always)]
    fn fsub(self, other: Self) -> Self {
        (f32::from_bits(self) - f32::from_bits(other)).to_bits()
    }
}

impl Fmul for u32 {
    #[inline(always)]
    fn fmul(self, other: Self) -> Self {
        (f32::from_bits(self) * f32::from_bits(other)).to_bits()
    }
}

impl Fdiv for u32 {
    #[inline(always)]
    fn fdiv(self, other: Self) -> Self {
        (f32::from_bits(self) / f32::from_bits(other)).to_bits()
    }
}

impl Fsqrt for u32 {
    #[inline(always)]
    fn fsqrt(self, _: Self) -> Self {
        f32::from_bits(self).sqrt().to_bits()
    }
}

impl Fsgnj for u32 {
    #[inline(always)]
    fn fsgnj(self, other: Self) -> Self {
        ((self << 1) >> 1) | ((other >> 31) << 31)
    }
}

impl Fsgnjn for u32 {
    #[inline(always)]
    fn fsgnjn(self, other: Self) -> Self {
        ((self << 1) >> 1) | (!(other >> 31) << 31)
    }
}

impl Fsgnjx for u32 {
    #[inline(always)]
    fn fsgnjx(self, other: Self) -> Self {
        self ^ ((other >> 31) << 31)
    }
}

impl Fmin for u32 {
    #[inline(always)]
    fn fmin(self, other: Self) -> Self {
        match f32::from_bits(self).total_cmp(&f32::from_bits(other)) {
            Ordering::Less => self,
            _ => other,
        }
    }
}

impl Fmax for u32 {
    #[inline(always)]
    fn fmax(self, other: Self) -> Self {
        match f32::from_bits(self).total_cmp(&f32::from_bits(other)) {
            Ordering::Greater => self,
            _ => other,
        }
    }
}

impl Feq for u32 {
    #[inline(always)]
    fn feq(self, other: Self) -> Self {
        (self == other) as Self
    }
}

impl Flt for u32 {
    #[inline(always)]
    fn flt(self, other: Self) -> Self {
        println!("{} < {}", self, other);
        (f32::from_bits(self) < f32::from_bits(other)) as Self
    }
}

impl Fle for u32 {
    #[inline(always)]
    fn fle(self, other: Self) -> Self {
        (f32::from_bits(self) <= f32::from_bits(other)) as Self
    }
}

impl Fcvtws for u32 {
    #[inline(always)]
    fn fcvtws(self, _: Self) -> Self {
        f32::from_bits(self) as i32 as u32
    }
}

impl Fcvtwus for u32 {
    #[inline(always)]
    fn fcvtwus(self, _: Self) -> Self {
        f32::from_bits(self) as u32
    }
}

impl Fmvxw for u32 {
    #[inline(always)]
    fn fmvxw(self, _: Self) -> Self {
        self
    }
}

impl Fclass for u32 {
    #[inline(always)]
    fn fclass(self, _: Self) -> Self {
        let f = f32::from_bits(self);
        match f.classify() {
            FpCategory::Infinite => {
                if f.is_sign_negative() {
                    1 << 0
                } else {
                    1 << 7
                }
            }
            FpCategory::Normal => {
                if f.is_sign_negative() {
                    1 << 1
                } else {
                    1 << 6
                }
            }
            FpCategory::Subnormal => {
                if f.is_sign_negative() {
                    1 << 2
                } else {
                    1 << 5
                }
            }
            FpCategory::Zero => {
                if f.is_sign_negative() {
                    1 << 3
                } else {
                    1 << 4
                }
            }
            FpCategory::Nan => {
                if f.to_bits() == 0x7fc00000 {
                    1 << 8
                } else {
                    1 << 9
                }
            }
        }
    }
}
