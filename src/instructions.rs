use crate::decode::{Shift, B, I, J, R, S, U, U10, U12, U3};
use crate::error::Error;
use crate::registers::{Registers, ZeroOrRegister};

const OPCODE_SIZE: u32 = 4;

macro_rules! def_uconst {
    ($($v:vis const $name:ident: $t:ty = $n:expr;)*) => {
        $(
            #[allow(clippy::unusual_byte_groupings)]
            $v const $name: $t = if let Some(n) = <$t>::new($n) {
                n
            } else {
                panic!(concat!("Value ", stringify!($n), " out of ", stringify!($t), " range"))
            };
        )*
    };
}

def_uconst! {
    const ADD: U10 = 0b0000000_000;
    const SUB: U10 = 0b0100000_000;
    const SLL: U10 = 0b0000000_001;
    const SLT: U10 = 0b0000000_010;
    const SLTU: U10 = 0b0000000_011;
    const XOR: U10 = 0b0000000_100;
    const SRL: U10 = 0b0000000_101;
    const SRA: U10 = 0b0100000_101;
    const OR: U10 = 0b0000000_110;
    const AND: U10 = 0b0000000_111;
    const ADDI: U3 = 0b000;
    const SLTI: U3 = 0b010;
    const SLTIU: U3 = 0b011;
    const XORI: U3 = 0b100;
    const ORI: U3 = 0b110;
    const ANDI: U3 = 0b111;
    const LB: U3 = 0b000;
    const LBU: U3 = 0b100;
    const LH: U3 = 0b001;
    const LHU: U3 = 0b101;
    const LW: U3 = 0b010;
    const SLLI: U10 = 0b0000000_001;
    const SRLI: U10 = 0b0000000_101;
    const SRAI: U10 = 0b0100000_101;
    const SB: U3 = 0b000;
    const SH: U3 = 0b001;
    const SW: U3 = 0b010;
    const BEQ: U3 = 0b000;
    const BNE: U3 = 0b001;
    const BLT: U3 = 0b100;
    const BGE: U3 = 0b101;
    const BLTU: U3 = 0b110;
    const BGEU: U3 = 0b111;
}

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
        ADD => u32::wrapping_add,
        SUB => u32::wrapping_sub,
        SLL => u32::wrapping_shl, // wrapping shl is already masking with (0b11111)
        SLT => {
            |a, b| unsafe { (core::mem::transmute::<_, i32>(a) < core::mem::transmute(b)) as u32 }
        }
        SLTU => |a, b| (a < b) as u32,
        XOR => std::ops::BitXor::bitxor,
        SRL => u32::wrapping_shr,
        SRA => |a, b| unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(a).wrapping_shr(b))
        },
        OR => std::ops::BitOr::bitor,
        AND => std::ops::BitAnd::bitand,
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
        ADDI => |a, b| a.wrapping_add(b.as_u32()),
        SLTI => |a, b| {
            let a: i32 = unsafe { core::mem::transmute(a) };
            let b = b.sign_extend() as i32;
            (a < b) as u32
        },
        SLTIU => |a, b| (a < b.as_u32()) as u32,
        XORI => |a, b| a ^ b.as_u32(),
        ORI => |a, b| a | b.as_u32(),
        ANDI => |a, b| a & b.as_u32(),
        _ => return Err(Error::InvalidOpCode),
    };

    exec(instruction, regs, f)
}

#[inline(always)]
pub(crate) fn execute_load(
    instruction: I,
    regs: &mut Registers<u32>,
    memory: &[u8],
) -> Result<(), Error> {
    use crate::mem::{self, Pod, I16, U16, U32};

    #[inline(always)]
    fn exec<T, F>(
        instruction: I,
        regs: &mut Registers<u32>,
        memory: &[u8],
        f: F,
    ) -> Result<(), Error>
    where
        T: Pod,
        F: Fn(T) -> u32,
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
        *regs.get_mut(dest_reg) = f(mem::read::<T>(memory, offset)?);
        Ok(())
    }

    match instruction.id() {
        LB => exec(instruction, regs, memory, |n: i8| unsafe {
            core::mem::transmute(n as i32)
        }),
        LBU => exec(instruction, regs, memory, |n: u8| n as u32),
        LH => exec(instruction, regs, memory, |n: I16| unsafe {
            core::mem::transmute(n.as_i16() as i32)
        }),
        LHU => exec(instruction, regs, memory, |n: U16| n.as_u16() as u32),
        LW => exec(instruction, regs, memory, |n: U32| n.as_u32()),
        _ => Err(Error::InvalidOpCode),
    }
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
        *regs.get_mut(reg) = pc.wrapping_add(OPCODE_SIZE);
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
        *regs.get_mut(reg) = pc.wrapping_add(OPCODE_SIZE);
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
        SLLI => |a, b| a.wrapping_shl(b),
        SRLI => |a, b| a.wrapping_shr(b),
        SRAI => |a, b| unsafe {
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
    use crate::mem::{self, Pod, U16, U32};

    #[inline(always)]
    fn exec<T, F>(
        instruction: S,
        regs: &mut Registers<u32>,
        memory: &mut [u8],
        f: F,
    ) -> Result<(), Error>
    where
        T: Pod,
        F: Fn(u32) -> T,
    {
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
        let offset = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32) as usize;
        mem::write(&f(src2), memory, offset)
    }

    match instruction.id() {
        SB => exec(instruction, regs, memory, |n| n as u8),
        SH => exec(instruction, regs, memory, |n| U16::new(n as u16)),
        SW => exec(instruction, regs, memory, U32::new),
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
        } else {
            *pc = pc.wrapping_add(OPCODE_SIZE);
        }
        Ok(())
    }

    let f: fn(u32, u32) -> bool = match instruction.id() {
        BEQ => |a, b| a == b,
        BNE => |a, b| a != b,
        BLT => |a, b| unsafe { core::mem::transmute::<_, i32>(a) < core::mem::transmute(b) },
        BGE => |a, b| unsafe { core::mem::transmute::<_, i32>(a) >= core::mem::transmute(b) },
        BLTU => |a, b| a < b,
        BGEU => |a, b| a >= b,
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
