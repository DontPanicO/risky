use std::cmp::Ordering;

use crate::error::Error;
use crate::mem;
use crate::registers::{Registers, ZeroOrRegister};

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
    fn id(&self) -> u32 {
        self.funct3.as_u32() + self.funct7.as_u32()
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
    fn id(&self) -> u32 {
        self.funct3.as_u32()
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
            prefix: unsafe { U7::new_unchecked((imm.as_u16() >> 7) as u8) },
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
    fn id(&self) -> u32 {
        self.funct3.as_u32() + self.prefix.as_u32()
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
                U12::new_unchecked((((value >> 25) & B7_MASK) | ((value >> 7) & B5_MASK)) as u16)
            },
            rs2: U5::new_truncate((value >> 20) as u8),
            rs1: U5::new_truncate((value >> 15) as u8),
            funct3: U3::new_truncate((value >> 12) as u8),
        }
    }

    #[inline(always)]
    fn id(&self) -> u32 {
        self.funct3.as_u32()
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
    fn id(&self) -> u32 {
        self.funct3.as_u32()
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
            imm: (value >> 12) & ((2 << 20) + 1),
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

#[inline(always)]
pub(crate) fn execute_math(instruction: R, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // add
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_add(src2);
        }
        1 => {
            // SLL (rs2 truncated)
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shl(src2);
        }
        2 | 3 => {
            // SLT/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            match src1.cmp(&src2) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        4 => {
            // XOR
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 ^ src2
        }
        5 => {
            // SRL (rs2 truncated)
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(src2);
        }
        6 => {
            // OR
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 | src2
        }
        7 => {
            // AND
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 & src2
        }
        32 => {
            // SUB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_sub(src2);
        }
        37 => {
            // SRA (rs2 truncated)
            let src1: i32 = unsafe {
                core::mem::transmute(
                    ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()).fetch(regs),
                )
            };
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(src2) as u32;
        }
        _ => return Err(Error::InvalidOpCode),
    };
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_mathi(instruction: I, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // ADDI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_add(instruction.imm.as_u32());
        }
        2 => {
            // SLTI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let immediate_signed = instruction.imm.sign_extend() as i32;
            let src1_signed: i32 = unsafe { core::mem::transmute(src1) };
            match src1_signed.cmp(&immediate_signed) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        3 => {
            // SLTIU
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            match src1.cmp(&instruction.imm.as_u32()) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        4 => {
            // XORI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 ^ instruction.imm.as_u32();
        }
        6 => {
            // ORI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 | instruction.imm.as_u32();
        }
        7 => {
            // ANDI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 & instruction.imm.as_u32();
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

// TODO: FIX memrxx calls (now reading from empty slice)
#[inline(always)]
pub(crate) fn execute_load(
    instruction: I,
    regs: &mut Registers<u32>,
    memory: &[u8],
) -> Result<(), Error> {
    match instruction.id() {
        0 | 4 => {
            // LB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.imm.as_u32();
            *dest = mem::memr8(memory, addr as usize)? as u32
        }
        1 | 5 => {
            // LH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.imm.as_u32();
            *dest = u16::from_le_bytes(mem::memr16(memory, addr as usize)?) as u32;
        }
        2 => {
            // LW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.imm.as_u32();
            *dest = u32::from_le_bytes(mem::memr32(memory, addr as usize)?);
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_jalr(
    instruction: I,
    regs: &mut Registers<u32>,
    pc: &mut u32,
    link: u8,
) -> Result<(), Error> {
    match unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }.fetch_mut(regs) {
        Some(dest) => {
            *dest = *pc + 4;
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            *pc += src1
                + unsafe { core::mem::transmute::<i16, u16>(instruction.imm.sign_extend()) } as u32;
        }
        None => {
            *pc = unsafe { ZeroOrRegister::decode_unchecked(link) }.fetch(regs);
        }
    };
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_shifti(instruction: Shift, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        1 => {
            // SLLI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shl(instruction.shamt.as_u32());
        }
        5 => {
            // SRLI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(instruction.shamt.as_u32());
        }
        68 => {
            // SRAI
            let src1: i32 = unsafe {
                core::mem::transmute(
                    ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()).fetch(regs),
                )
            };
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = unsafe { core::mem::transmute(src1.wrapping_shr(instruction.shamt.as_u32())) };
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_store(
    instruction: S,
    regs: &mut Registers<u32>,
    memory: &mut [u8],
) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // SB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs) as u8;
            let addr = src1 + instruction.imm.as_u32();
            mem::memw(&src2.to_le_bytes(), memory, addr as usize)?;
        }
        1 => {
            // SH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs) as u16;
            let addr = src1 + instruction.imm.as_u32();
            mem::memw(&src2.to_le_bytes(), memory, addr as usize)?;
        }
        2 => {
            // SW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            let addr = src1 + instruction.imm.as_u32();
            mem::memw(&src2.to_le_bytes(), memory, addr as usize)?;
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_branch(
    instruction: B,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // BEQ
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            if src1 == src2 {
                let offset = instruction.imm.sign_extend() as i32;
                *pc = pc.saturating_add_signed(offset);
            }
        }
        1 => {
            // BNE
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            match src1.cmp(&src2) {
                Ordering::Less | Ordering::Greater => {
                    let offset = instruction.imm.sign_extend() as i32;
                    *pc = pc.saturating_add_signed(offset);
                }
                _ => {}
            }
        }
        4 | 6 => {
            // BLT/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            if src1 < src2 {
                let offset = instruction.imm.sign_extend() as i32;
                *pc = pc.saturating_add_signed(offset);
            }
        }
        5 | 7 => {
            // BGE/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            match src1.cmp(&src2) {
                Ordering::Equal | Ordering::Greater => {
                    let offset = instruction.imm.sign_extend() as i32;
                    *pc = pc.saturating_add_signed(offset);
                }
                _ => {}
            }
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_lui(instruction: U, regs: &mut Registers<u32>) -> Result<(), Error> {
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = instruction.imm.wrapping_shl(12);
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_auipc(
    instruction: U,
    regs: &mut Registers<u32>,
    pc: u32,
) -> Result<(), Error> {
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = pc + instruction.imm.wrapping_shl(12);
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_jal(
    instruction: J,
    regs: &mut Registers<u32>,
    pc: &mut u32,
    link: &mut u8,
) -> Result<(), Error> {
    match unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }.fetch_mut(regs) {
        Some(dest) => {
            *link = instruction.rd.as_u8();
            *dest = *pc + 4;
            let offset = instruction.imm.sign_extend();
            *pc = pc.saturating_add_signed(offset);
        }
        None => {
            let offset = instruction.imm.sign_extend();
            *pc = pc.saturating_add_signed(offset);
        }
    };
    Ok(())
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
impl_u16!(U12, 12,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7);
impl_u16!(U13, 13,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7,
          U12 > as_u16 => from_u12);
impl_u32!(U21, 21,
          U2  > as_u8  => from_u2,
          U3  > as_u8  => from_u3,
          U4  > as_u8  => from_u4,
          U5  > as_u8  => from_u5,
          U7  > as_u8  => from_u7,
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
            unsafe { core::mem::transmute(0b1111111111111110u16) }
        );
        assert_eq!(
            U13::new_truncate(0b0111111111110u16).sign_extend(),
            unsafe { core::mem::transmute(0b0000111111111110u16) }
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
