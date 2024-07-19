use crate::num::{Bitcast, Unsigned};

pub trait Beq {
    fn beq(self, other: Self) -> bool;
}

pub trait Bne {
    fn bne(self, other: Self) -> bool;
}

pub trait Blt {
    fn blt(self, other: Self) -> bool;
}

pub trait Bge {
    fn bge(self, other: Self) -> bool;
}

pub trait Bltu {
    fn bltu(self, other: Self) -> bool;
}

pub trait Bgeu {
    fn bgeu(self, other: Self) -> bool;
}

#[allow(unused)]
pub trait BaseBranch: Beq + Bne + Blt + Bge + Bltu + Bgeu {}

impl<T> Beq for T
where
    T: core::cmp::Eq,
{
    #[inline(always)]
    fn beq(self, other: Self) -> bool {
        self == other
    }
}

impl<T> Bne for T
where
    T: core::cmp::Eq,
{
    #[inline(always)]
    fn bne(self, other: Self) -> bool {
        self != other
    }
}

impl<T> Blt for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: core::cmp::Ord,
{
    #[inline(always)]
    fn blt(self, other: Self) -> bool {
        <T as Bitcast<<T as Unsigned>::Signed>>::bitcast(self)
            < <T as Bitcast<<T as Unsigned>::Signed>>::bitcast(other)
    }
}

impl<T> Bge for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: core::cmp::Ord,
{
    #[inline(always)]
    fn bge(self, other: Self) -> bool {
        <T as Bitcast<<T as Unsigned>::Signed>>::bitcast(self)
            >= <T as Bitcast<<T as Unsigned>::Signed>>::bitcast(other)
    }
}

impl<T> Bltu for T
where
    T: core::cmp::Ord,
{
    #[inline(always)]
    fn bltu(self, other: Self) -> bool {
        self < other
    }
}

impl<T> Bgeu for T
where
    T: core::cmp::Ord,
{
    #[inline(always)]
    fn bgeu(self, other: Self) -> bool {
        self >= other
    }
}
