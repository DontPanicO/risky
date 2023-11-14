use std::cmp::Ordering;

use crate::decode::{Shift, B, I, J, R, S, U};
use crate::error::Error;
use crate::mem;
use crate::registers::{Registers, ZeroOrRegister};

#[inline(always)]
pub(crate) fn execute_math(instruction: R, regs: &mut Registers<u32>) -> Result<(), Error> {
    #[inline(always)]
    fn exec<F>(instruction: R, regs: &mut Registers<u32>, f: F) -> Result<(), Error>
    where
        F: Fn(u32, u32) -> u32,
    {
        match instruction.rd.into() {
            ZeroOrRegister::Zero => Err(Error::InvalidOpCode),
            ZeroOrRegister::Register(reg) => {
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                *regs.get_mut(reg) = f(src1, src2);
                Ok(())
            }
        }
    }

    let f = match instruction.id() {
        // ADD
        0 => u32::wrapping_add,
        // SLL (rs2 truncated)
        1 => u32::wrapping_shl, // wrapping shl is already masking with (0b11111)
        // SLT
        2 => |a, b| unsafe { (core::mem::transmute::<_, i32>(a) < core::mem::transmute(b)) as u32 },
        // SLTU
        // NOTE: SLTU rd, x0, rs2 sets rd to 1 if rs2 is not equal to zero, otherwise sets rd to zero
        //       (assembler pseudoinstruction SNEZ rd, rs).
        3 => {
            return match instruction.rd.into() {
                ZeroOrRegister::Zero => Err(Error::InvalidOpCode),
                ZeroOrRegister::Register(dest) => {
                    let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                    *regs.get_mut(dest) = match instruction.rs1.into() {
                        ZeroOrRegister::Zero => src2 != 0,
                        ZeroOrRegister::Register(src1_reg) => src1_reg.fetch(regs) < src2,
                    } as u32;

                    Ok(())
                }
            };
        }
        // XOR
        4 => std::ops::BitXor::bitxor,
        // SRL (rs2 truncated)
        5 => u32::wrapping_shr,
        // OR
        6 => std::ops::BitOr::bitor,
        // AND
        7 => std::ops::BitAnd::bitand,
        // SUB
        32 => u32::wrapping_sub,
        // SRA (rs2 truncated)
        37 => |a, b| unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(a).wrapping_shr(b))
        },
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, f)
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
