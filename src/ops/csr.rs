use crate::{decode::U5, num::As};

pub trait Csrrw {
    fn csrrw(src: Self, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait Csrrs {
    fn csrrs(src: Self, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait Csrrc {
    fn csrrc(src: Self, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait Csrrwi {
    fn csrrwi(imm: U5, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait Csrrsi {
    fn csrrsi(imm: U5, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait Csrrci {
    fn csrrci(imm: U5, dest: &mut Self, csr: &mut Self) -> Self;
}

pub trait BaseCsr: Csrrw + Csrrs + Csrrc + Csrrwi + Csrrsi + Csrrci {}

impl<T: Copy> Csrrw for T {
    #[inline(always)]
    fn csrrw(src: Self, dest: &mut Self, csr: &mut Self) -> Self {
        *dest = *csr;
        src
    }
}

impl<T: Copy + std::ops::BitOr<Output = T>> Csrrs for T {
    #[inline(always)]
    fn csrrs(src: Self, dest: &mut Self, csr: &mut Self) -> Self {
        // For both CSRRS and CSRRC, if rs1=x0, then the instruction will not write to the CSR at all, and
        // so shall not cause any of the side effects that might otherwise occur on a CSR write, such as raising
        // illegal instruction exceptions on accesses to read-only CSRs.
        *dest = *csr;
        *csr | src
    }
}

impl<T: Copy + std::ops::BitAnd<Output = T> + std::ops::Not<Output = T>> Csrrc for T {
    #[inline(always)]
    fn csrrc(src: Self, dest: &mut Self, csr: &mut Self) -> Self {
        // For both CSRRS and CSRRC, if rs1=x0, then the instruction will not write to the CSR at all, and
        // so shall not cause any of the side effects that might otherwise occur on a CSR write, such as raising
        // illegal instruction exceptions on accesses to read-only CSRs.
        *dest = *csr;
        *csr & !src
    }
}

impl<T> Csrrwi for T
where
    T: Copy,
    u8: As<T>,
{
    #[inline(always)]
    fn csrrwi(imm: U5, dest: &mut Self, csr: &mut Self) -> Self {
        *dest = *csr;
        imm.as_u8().r#as()
    }
}

impl<T> Csrrsi for T
where
    T: Copy,
    T: std::ops::BitOr<Output = T>,
    u8: As<T>,
{
    #[inline(always)]
    fn csrrsi(imm: U5, dest: &mut Self, csr: &mut Self) -> Self {
        // For both CSRRS and CSRRC, if rs1=x0, then the instruction will not write to the CSR at all, and
        // so shall not cause any of the side effects that might otherwise occur on a CSR write, such as raising
        // illegal instruction exceptions on accesses to read-only CSRs.
        *dest = *csr;
        *csr | imm.as_u8().r#as()
    }
}

impl<T> Csrrci for T
where
    T: Copy,
    T: std::ops::BitAnd<Output = T>,
    T: std::ops::Not<Output = T>,
    u8: As<T>,
{
    #[inline(always)]
    fn csrrci(imm: U5, dest: &mut Self, csr: &mut Self) -> Self {
        // For both CSRRS and CSRRC, if rs1=x0, then the instruction will not write to the CSR at all, and
        // so shall not cause any of the side effects that might otherwise occur on a CSR write, such as raising
        // illegal instruction exceptions on accesses to read-only CSRs.
        *dest = *csr;
        *csr & !imm.as_u8().r#as()
    }
}

impl<T> BaseCsr for T
where
    T: Copy,
    T: std::ops::BitOr<Output = T>,
    T: std::ops::BitAnd<Output = T>,
    T: std::ops::Not<Output = T>,
    u8: As<T>,
{
}
