use crate::{
    error::Error,
    mem::{read, write, I16, I32, I64, U16, U32, U64},
};

pub trait Lb: Sized {
    fn lb(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Lbu: Sized {
    fn lbu(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Lh: Sized {
    fn lh(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Lhu: Sized {
    fn lhu(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Lw: Sized {
    fn lw(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Lwu: Sized {
    fn lwu(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Ld: Sized {
    fn ld(memory: &[u8], addr: usize) -> Result<Self, Error>;
}

pub trait Sb: Sized {
    fn sb(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error>;
}

pub trait Sh: Sized {
    fn sh(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error>;
}

pub trait Sw: Sized {
    fn sw(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error>;
}

pub trait Sd: Sized {
    fn sd(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error>;
}

pub trait BaseLoad: Lb + Lbu + Lh + Lhu + Lw + Lwu + Ld {}
pub trait BaseStore: Sb + Sh + Sw + Sd {}

impl Lb for u32 {
    #[inline(always)]
    fn lb(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<i8>(memory, addr)? as i32) })
    }
}

impl Lbu for u32 {
    #[inline(always)]
    fn lbu(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(read::<u8>(memory, addr)? as u32)
    }
}

impl Lh for u32 {
    #[inline(always)]
    fn lh(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<I16>(memory, addr)?.as_i16() as i32) })
    }
}

impl Lhu for u32 {
    #[inline(always)]
    fn lhu(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(read::<U16>(memory, addr)?.as_u16() as u32)
    }
}

impl Lw for u32 {
    #[inline(always)]
    fn lw(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<I32>(memory, addr)?.as_i32()) })
    }
}

impl Lb for u64 {
    #[inline(always)]
    fn lb(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<i8>(memory, addr)? as i64) })
    }
}

impl Lbu for u64 {
    #[inline(always)]
    fn lbu(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(read::<u8>(memory, addr)? as u64)
    }
}

impl Lh for u64 {
    #[inline(always)]
    fn lh(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<I16>(memory, addr)?.as_i16() as i64) })
    }
}

impl Lhu for u64 {
    #[inline(always)]
    fn lhu(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(read::<U16>(memory, addr)?.as_u16() as u64)
    }
}

impl Lw for u64 {
    #[inline(always)]
    fn lw(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<I32>(memory, addr)?.as_i32() as i64) })
    }
}

impl Lwu for u64 {
    #[inline(always)]
    fn lwu(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(read::<U32>(memory, addr)?.as_u32() as u64)
    }
}

impl Ld for u64 {
    #[inline(always)]
    fn ld(memory: &[u8], addr: usize) -> Result<Self, Error> {
        Ok(unsafe { core::mem::transmute(read::<I64>(memory, addr)?.as_i64()) })
    }
}

impl Sb for u32 {
    #[inline(always)]
    fn sb(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&(src as u8), memory, addr)
    }
}

impl Sh for u32 {
    #[inline(always)]
    fn sh(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&U16::new(src as u16), memory, addr)
    }
}

impl Sw for u32 {
    #[inline(always)]
    fn sw(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&U32::new(src), memory, addr)
    }
}

impl Sb for u64 {
    #[inline(always)]
    fn sb(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&(src as u8), memory, addr)
    }
}

impl Sh for u64 {
    #[inline(always)]
    fn sh(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&U16::new(src as u16), memory, addr)
    }
}
impl Sw for u64 {
    #[inline(always)]
    fn sw(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&U32::new(src as u32), memory, addr)
    }
}
impl Sd for u64 {
    #[inline(always)]
    fn sd(src: Self, memory: &mut [u8], addr: usize) -> Result<(), Error> {
        write(&U64::new(src), memory, addr)
    }
}
