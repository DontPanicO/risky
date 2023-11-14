use crate::decode::{Shift, B, I, J, R, S, U, U12};
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
    #[inline(always)]
    fn exec<F>(instruction: I, regs: &mut Registers<u32>, f: F) -> Result<(), Error>
    where
        F: Fn(u32, U12) -> u32,
    {
        match instruction.rd.into() {
            ZeroOrRegister::Zero => Err(Error::InvalidOpCode),
            ZeroOrRegister::Register(reg) => {
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                *regs.get_mut(reg) = f(src1, instruction.imm);
                Ok(())
            }
        }
    }

    let f: fn(u32, U12) -> u32 = match instruction.id() {
        // ADDI
        0 => |a, b| a.wrapping_add(b.as_u32()),
        // SLTI
        2 => |a, b| {
            let a: i32 = unsafe { core::mem::transmute(a) };
            let b = b.sign_extend() as i32;
            (a < b) as u32
        },
        // SLTIU
        // NOTE: SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals zero, otherwise sets rd to 0
        //       (assembler pseudoinstruction SEQZ rd, rs).
        //       Ignored because rs1 < 1 can only be 0.
        3 => |a, b| (a < b.as_u32()) as u32,
        // XORI
        4 => |a, b| a ^ b.as_u32(),
        // ORI
        5 => |a, b| a | b.as_u32(),
        // ANDI
        7 => |a, b| a & b.as_u32(),
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, f)
}

// TODO: FIX memrxx calls (now reading from empty slice)
#[inline(always)]
pub(crate) fn execute_load(
    instruction: I,
    regs: &mut Registers<u32>,
    memory: &[u8],
) -> Result<(), Error> {
    #[inline(always)]
    fn exec<F>(instruction: I, regs: &mut Registers<u32>, memory: &[u8], f: F) -> Result<(), Error>
    where
        F: Fn(&[u8], usize) -> Result<u32, Error>,
    {
        let dest_reg =
            if let ZeroOrRegister::Register(reg) = ZeroOrRegister::from_u5(instruction.rd) {
                reg
            } else {
                return Err(Error::InvalidOpCode);
            };
        let offset = ZeroOrRegister::from_u5(instruction.rs1)
            .fetch(regs)
            .wrapping_add_signed(instruction.imm.sign_extend() as i32)
            as usize;
        *regs.get_mut(dest_reg) = f(memory, offset)?;
        Ok(())
    }

    let f: fn(&[u8], usize) -> Result<u32, Error> = match instruction.id() {
        // LB
        0b000 => |memory, offset| {
            mem::memr8(memory, offset)
                .map(|n| unsafe { core::mem::transmute(core::mem::transmute::<_, i8>(n) as i32) })
        },
        // LBU
        0b100 => |memory, offset| mem::memr8(memory, offset).map(|n| n as u32),
        // LH
        0b001 => |memory, offset| {
            mem::memr16(memory, offset)
                .map(|n| unsafe { core::mem::transmute(i16::from_le_bytes(n) as i32) })
        },
        // LHU
        0b101 => |memory, offset| mem::memr16(memory, offset).map(|n| u16::from_le_bytes(n) as u32),
        // LW
        0b011 => |memory, offset| mem::memr32(memory, offset).map(u32::from_le_bytes),
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, memory, f)
}

#[inline(always)]
pub(crate) fn execute_jal(
    instruction: J,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
    //       address is not aligned to a four-byte boundary. (???)

    if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
        *regs.get_mut(reg) = *pc + 4;
    }

    *pc = (*pc).wrapping_add_signed(instruction.imm.sign_extend());

    Ok(())
}

#[inline(always)]
pub(crate) fn execute_jalr(
    instruction: I,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
    //       address is not aligned to a four-byte boundary. (???)

    let next = ZeroOrRegister::from_u5(instruction.rs1)
        .fetch(regs)
        .wrapping_add_signed(instruction.imm.sign_extend() as i32)
        & !1;

    if let ZeroOrRegister::Register(reg) = ZeroOrRegister::from_u5(instruction.rd) {
        *regs.get_mut(reg) = *pc + 4;
    }

    *pc = next;

    Ok(())
}

#[inline(always)]
pub(crate) fn execute_shifti(instruction: Shift, regs: &mut Registers<u32>) -> Result<(), Error> {
    #[inline(always)]
    fn exec<F>(instruction: Shift, regs: &mut Registers<u32>, f: F) -> Result<(), Error>
    where
        F: Fn(u32, u32) -> u32,
    {
        let dest_reg = if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
            reg
        } else {
            return Err(Error::InvalidOpCode);
        };
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        *regs.get_mut(dest_reg) = f(src1, instruction.shamt.as_u32());

        Ok(())
    }

    let f: fn(u32, u32) -> u32 = match instruction.id() {
        // SLLI
        1 => |a, b| a.wrapping_shl(b),
        // SRLI
        5 => |a, b| a.wrapping_shr(b),
        // SRAI
        68 => |a, b| unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(a).wrapping_shr(b))
        },
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, f)
}

#[inline(always)]
pub(crate) fn execute_store(
    instruction: S,
    regs: &mut Registers<u32>,
    memory: &mut [u8],
) -> Result<(), Error> {
    #[inline(always)]
    fn exec<const N: usize, F>(
        instruction: S,
        regs: &mut Registers<u32>,
        memory: &mut [u8],
        f: F,
    ) -> Result<(), Error>
    where
        F: Fn(u32) -> [u8; N],
    {
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
        let offset = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32) as usize;
        mem::memw(&f(src2), memory, offset)
    }

    match instruction.id() {
        // SB
        0b000 => exec(instruction, regs, memory, |n| u8::to_le_bytes(n as u8)),
        // SH
        0b001 => exec(instruction, regs, memory, |n| u16::to_le_bytes(n as u16)),
        // SW
        0b010 => exec(instruction, regs, memory, u32::to_le_bytes),
        _ => Err(Error::InvalidOpCode),
    }
}

#[inline(always)]
pub(crate) fn execute_branch(
    instruction: B,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    #[inline(always)]
    fn exec<F>(instruction: B, regs: &mut Registers<u32>, pc: &mut u32, f: F) -> Result<(), Error>
    where
        F: Fn(u32, u32) -> bool,
    {
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
        if f(src1, src2) {
            *pc = pc.wrapping_add_signed(instruction.imm.sign_extend() as i32);
        }
        Ok(())
    }

    let f: fn(u32, u32) -> bool = match instruction.id() {
        // BEQ
        0 => |a, b| a == b,
        // BNE
        1 => |a, b| a != b,
        // BLT
        4 => |a, b| a < b,
        // BGE
        5 => |a, b| a >= b,
        // BLTU
        6 => |a, b| unsafe { core::mem::transmute::<_, i32>(a) < core::mem::transmute(b) },
        // BLTU
        7 => |a, b| unsafe { core::mem::transmute::<_, i32>(a) >= core::mem::transmute(b) },
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, pc, f)
}

#[inline(always)]
pub(crate) fn execute_lui(instruction: U, regs: &mut Registers<u32>) -> Result<(), Error> {
    let dest = ZeroOrRegister::from_u5(instruction.rd)
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
    let dest = ZeroOrRegister::from_u5(instruction.rd)
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = pc.wrapping_add(instruction.imm);
    Ok(())
}
