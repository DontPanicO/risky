use std::cmp::Ordering;

use crate::decode::{Shift, B, I, J, R, S, U};
use crate::error::Error;
use crate::mem;
use crate::registers::{Registers, ZeroOrRegister};

const OPCODE_SIZE: u32 = 4;
const B5_MASK: u32 = 0b11111;

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
                & B5_MASK;
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
                & B5_MASK;
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
                & B5_MASK;
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
                & B5_MASK;
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
                & B5_MASK;
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
                & B5_MASK;
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
                & B5_MASK;
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
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
            *dest = mem::memr8(memory, addr as usize)? as u32
        }
        1 | 5 => {
            // LH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
            *dest = u16::from_le_bytes(mem::memr16(memory, addr as usize)?) as u32;
        }
        2 => {
            // LW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
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
) -> Result<(), Error> {
    match unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }.fetch_mut(regs) {
        Some(dest) => {
            *dest = pc.wrapping_add(OPCODE_SIZE);
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            *pc = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32) & !0b1;
        }
        None => {
            *pc = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
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
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
            mem::memw(&src2.to_le_bytes(), memory, addr as usize)?;
        }
        1 => {
            // SH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }
                .fetch(regs) as u16;
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
            mem::memw(&src2.to_le_bytes(), memory, addr as usize)?;
        }
        2 => {
            // SW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1.as_u8()) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2.as_u8()) }.fetch(regs);
            let addr = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32);
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
                *pc = pc.wrapping_add_signed(offset);
            } else {
                *pc = pc.wrapping_add(OPCODE_SIZE);
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
                    *pc = pc.wrapping_add_signed(offset);
                }
                _ => *pc = pc.wrapping_add(OPCODE_SIZE),
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
                *pc = pc.wrapping_add_signed(offset);
            } else {
                *pc = pc.wrapping_add(OPCODE_SIZE);
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
                    *pc = pc.wrapping_add_signed(offset);
                }
                _ => *pc = pc.wrapping_add(OPCODE_SIZE),
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
    *dest = instruction.imm;
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
) -> Result<(), Error> {
    match unsafe { ZeroOrRegister::decode_unchecked(instruction.rd.as_u8()) }.fetch_mut(regs) {
        Some(dest) => {
            *dest = pc.wrapping_add(OPCODE_SIZE);
            let offset = instruction.imm.sign_extend();
            *pc = pc.wrapping_add_signed(offset);
        }
        None => {
            let target = instruction.imm.sign_extend();
            *pc = pc.wrapping_add_signed(target);
        }
    };
    Ok(())
}
