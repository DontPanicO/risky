use crate::decode::{Shift, B, I, J, R, S, U, U12, U5};
use crate::error::Error;
use crate::instruction_ids::*;
use crate::num::Unsigned;
use crate::ops::*;
use crate::registers::{CsrRegisters, Registers, Zero, ZeroOrRegister};

const OPCODE_SIZE: u32 = 4;

pub trait Math: Sized {
    fn math(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait MathW: Sized {
    fn mathw(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait MathI: Sized {
    fn mathi(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait MathIW: Sized {
    fn mathiw(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait ShiftI: Sized {
    fn shifti(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait ShiftIW: Sized {
    fn shiftiw(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait Load: Sized {
    fn load(instruction: I, regs: &mut Registers<Self>, memory: &[u8]) -> Result<(), Error>;
}

pub trait Store: Sized {
    fn store(instruction: S, regs: &mut Registers<Self>, memory: &mut [u8]) -> Result<(), Error>;
}

pub trait Branch: Sized {
    fn branch(instruction: B, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

pub trait Jal: Sized {
    fn jal(instruction: J, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

pub trait Jalr: Sized {
    fn jalr(instruction: I, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

pub trait Lui: Sized {
    fn lui(instruction: U, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait Auipc: Sized {
    fn auipc(instruction: U, regs: &mut Registers<Self>, pc: Self) -> Result<(), Error>;
}

pub trait Csr: Sized {
    fn csr(
        instruction: I,
        regs: &mut Registers<Self>,
        csrs: &mut CsrRegisters<Self>,
    ) -> Result<(), Error>;
}

pub trait FloatS: Sized {
    fn floats(instruction: R, fregs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait BaseInstruction:
    Math + MathI + ShiftI + Load + Store + Branch + Jal + Jalr + Lui + Auipc
{
}

impl BaseInstruction for u32 {}
impl BaseInstruction for u64 {}

impl<T: Copy + BaseMath + Zero> Math for T {
    #[inline(always)]
    fn math(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, Self) -> Self = match instruction.id() {
            ADD => Add::add,
            SUB => Sub::sub,
            SLL => Sll::sll,
            SLT => Slt::slt,
            SLTU => Sltu::sltu,
            XOR => Xor::xor,
            SRL => Srl::srl,
            SRA => Sra::sra,
            OR => Or::or,
            AND => And::and,
            MUL => Mul::mul,
            MULH => Mulh::mulh,
            MULHSU => Mulhsu::mulhsu,
            MULHU => Mulhu::mulhu,
            DIV => Div::div,
            DIVU => Divu::divu,
            REM => Rem::rem,
            REMU => Remu::remu,
            _ => return Err(Error::InvalidOpCode),
        };

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
}

impl<T: Copy + BaseMath + Zero> MathI for T {
    #[inline(always)]
    fn mathi(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, U12) -> Self = match instruction.id() {
            ADDI => Addi::addi,
            SLTI => Slti::slti,
            SLTIU => Sltiu::sltiu,
            XORI => Xori::xori,
            ORI => Ori::ori,
            ANDI => Andi::andi,
            _ => return Err(Error::InvalidOpCode),
        };

        match instruction.rd.into() {
            ZeroOrRegister::Zero => Err(Error::InvalidOpCode),
            ZeroOrRegister::Register(reg) => {
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                *regs.get_mut(reg) = f(src1, instruction.imm);
                Ok(())
            }
        }
    }
}

impl<T: Copy + BaseMath + Zero> ShiftI for T {
    #[inline(always)]
    fn shifti(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, U5) -> Self = match instruction.id() {
            SLLI => Slli::slli,
            SRLI => Srli::srli,
            SRAI => Srai::srai,
            _ => return Err(Error::InvalidOpCode),
        };

        let dest_reg = if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
            reg
        } else {
            return Err(Error::InvalidOpCode);
        };
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        *regs.get_mut(dest_reg) = f(src1, instruction.shamt);

        Ok(())
    }
}

macro_rules! impl_branch {
    ($t:ty) => {
        impl Branch for $t {
            #[inline(always)]
            fn branch(
                instruction: B,
                regs: &mut Registers<Self>,
                pc: &mut Self,
            ) -> Result<(), Error> {
                let f: fn($t, $t) -> bool = match instruction.id() {
                    BEQ => Beq::beq,
                    BNE => Bne::bne,
                    BLT => Blt::blt,
                    BGE => Bge::bge,
                    BLTU => Bltu::bltu,
                    BGEU => Bgeu::bgeu,
                    _ => return Err(Error::InvalidOpCode),
                };
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                if f(src1, src2) {
                    *pc = pc.wrapping_add_signed(
                        instruction.imm.sign_extend() as <$t as Unsigned>::Signed
                    );
                } else {
                    *pc = pc.wrapping_add(OPCODE_SIZE as $t);
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_jal {
    ($t:ty) => {
        impl Jal for $t {
            #[inline(always)]
            fn jal(instruction: J, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error> {
                // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
                //       address is not aligned to a four-byte boundary. (???)
                if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
                    *regs.get_mut(reg) = pc.wrapping_add(OPCODE_SIZE as $t);
                }
                *pc = (*pc)
                    .wrapping_add_signed(instruction.imm.sign_extend() as <$t as Unsigned>::Signed);

                Ok(())
            }
        }
    };
}

macro_rules! impl_jalr {
    ($t:ty) => {
        impl Jalr for $t {
            #[inline(always)]
            fn jalr(
                instruction: I,
                regs: &mut Registers<Self>,
                pc: &mut Self,
            ) -> Result<(), Error> {
                // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
                //       address is not aligned to a four-byte boundary. (???)
                let next = ZeroOrRegister::from_u5(instruction.rs1)
                    .fetch(regs)
                    .wrapping_add_signed(instruction.imm.sign_extend() as <$t as Unsigned>::Signed)
                    & !1;
                if let ZeroOrRegister::Register(reg) = ZeroOrRegister::from_u5(instruction.rd) {
                    *regs.get_mut(reg) = pc.wrapping_add(OPCODE_SIZE as $t);
                }
                *pc = next;
                Ok(())
            }
        }
    };
}

impl_branch!(u32);
impl_branch!(u64);
impl_jal!(u32);
impl_jal!(u64);
impl_jalr!(u32);
impl_jalr!(u64);

impl MathW for u64 {
    #[inline(always)]
    fn mathw(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, Self) -> Self = match instruction.id() {
            ADDW => Addw::addw,
            SUBW => Subw::subw,
            SLLW => Sllw::sllw,
            SRLW => Srlw::srlw,
            SRAW => Sraw::sraw,
            _ => return Err(Error::InvalidOpCode),
        };
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
}

impl MathIW for u64 {
    #[inline(always)]
    fn mathiw(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, U12) -> Self = match instruction.id() {
            ADDIW => Addiw::addiw,
            _ => return Err(Error::InvalidOpCode),
        };
        match instruction.rd.into() {
            ZeroOrRegister::Zero => Err(Error::InvalidOpCode),
            ZeroOrRegister::Register(reg) => {
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                *regs.get_mut(reg) = f(src1, instruction.imm);
                Ok(())
            }
        }
    }
}

impl ShiftIW for u64 {
    #[inline(always)]
    fn shiftiw(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f: fn(Self, Self) -> Self = match instruction.id() {
            SLLIW => Slliw::slliw,
            SRLIW => Srliw::srliw,
            SRAIW => Sraiw::sraiw,
            _ => return Err(Error::InvalidOpCode),
        };
        let dest_reg = if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
            reg
        } else {
            return Err(Error::InvalidOpCode);
        };
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        *regs.get_mut(dest_reg) = f(src1, instruction.shamt.as_u64());

        Ok(())
    }
}

impl Load for u32 {
    #[inline(always)]
    fn load(instruction: I, regs: &mut Registers<Self>, memory: &[u8]) -> Result<(), Error> {
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
        *regs.get_mut(dest_reg) = match instruction.id() {
            LB => Lb::lb(memory, offset)?,
            LBU => Lbu::lbu(memory, offset)?,
            LH => Lh::lh(memory, offset)?,
            LHU => Lhu::lhu(memory, offset)?,
            LW => Lw::lw(memory, offset)?,
            _ => return Err(Error::InvalidOpCode),
        };
        Ok(())
    }
}

impl Load for u64 {
    #[inline(always)]
    fn load(instruction: I, regs: &mut Registers<Self>, memory: &[u8]) -> Result<(), Error> {
        let dest_reg =
            if let ZeroOrRegister::Register(reg) = ZeroOrRegister::from_u5(instruction.rd) {
                reg
            } else {
                return Err(Error::InvalidOpCode);
            };
        let offset = ZeroOrRegister::from_u5(instruction.rs1)
            .fetch(regs)
            .wrapping_add_signed(instruction.imm.sign_extend() as i64)
            as usize;
        *regs.get_mut(dest_reg) = match instruction.id() {
            LB => Lb::lb(memory, offset)?,
            LBU => Lbu::lbu(memory, offset)?,
            LH => Lh::lh(memory, offset)?,
            LHU => Lhu::lhu(memory, offset)?,
            LW => Lw::lw(memory, offset)?,
            LWU => Lwu::lwu(memory, offset)?,
            LD => Ld::ld(memory, offset)?,
            _ => return Err(Error::InvalidOpCode),
        };
        Ok(())
    }
}

impl Store for u32 {
    #[inline(always)]
    fn store(instruction: S, regs: &mut Registers<Self>, memory: &mut [u8]) -> Result<(), Error> {
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
        let offset = src1.wrapping_add_signed(instruction.imm.sign_extend() as i32) as usize;
        match instruction.id() {
            SB => Sb::sb(src2, memory, offset),
            SH => Sh::sh(src2, memory, offset),
            SW => Sw::sw(src2, memory, offset),
            _ => Err(Error::InvalidOpCode),
        }
    }
}

impl Store for u64 {
    #[inline(always)]
    fn store(instruction: S, regs: &mut Registers<Self>, memory: &mut [u8]) -> Result<(), Error> {
        let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
        let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
        let offset = src1.wrapping_add_signed(instruction.imm.sign_extend() as i64) as usize;
        match instruction.id() {
            SB => Sb::sb(src2, memory, offset),
            SH => Sh::sh(src2, memory, offset),
            SW => Sw::sw(src2, memory, offset),
            SD => Sd::sd(src2, memory, offset),
            _ => Err(Error::InvalidOpCode),
        }
    }
}

impl Lui for u32 {
    #[inline(always)]
    fn lui(instruction: U, regs: &mut Registers<Self>) -> Result<(), Error> {
        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest = instruction.imm;
        Ok(())
    }
}

impl Lui for u64 {
    #[inline(always)]
    fn lui(instruction: U, regs: &mut Registers<Self>) -> Result<(), Error> {
        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest =
            unsafe { core::mem::transmute(core::mem::transmute::<_, i32>(instruction.imm) as i64) };
        Ok(())
    }
}

impl Auipc for u32 {
    #[inline(always)]
    fn auipc(instruction: U, regs: &mut Registers<Self>, pc: Self) -> Result<(), Error> {
        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest = pc.wrapping_add(instruction.imm);
        Ok(())
    }
}

impl Auipc for u64 {
    #[inline(always)]
    fn auipc(instruction: U, regs: &mut Registers<Self>, pc: Self) -> Result<(), Error> {
        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest = pc.wrapping_add(unsafe {
            core::mem::transmute(core::mem::transmute::<_, i32>(instruction.imm) as i64)
        });
        Ok(())
    }
}

impl<T: Copy + Zero + BaseCsr> Csr for T {
    #[inline(always)]
    fn csr(
        instruction: I,
        regs: &mut Registers<Self>,
        csrs: &mut CsrRegisters<Self>,
    ) -> Result<(), Error> {
        if let ZeroOrRegister::Register(reg) = instruction.rs1.into() {
            let csr = csrs
                .get_mut(instruction.imm.as_u16() as usize)
                .ok_or(Error::InvalidOpCode)?;
            let src = reg.fetch(regs);
            let dest = ZeroOrRegister::from_u5(instruction.rd)
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *csr = match instruction.id() {
                CSRRW => Csrrw::csrrw(src, dest, csr),
                CSRRS => Csrrs::csrrs(src, dest, csr),
                CSRRC => Csrrc::csrrc(src, dest, csr),
                CSRRWI => Csrrwi::csrrwi(instruction.rs1, dest, csr),
                CSRRSI => Csrrsi::csrrsi(instruction.rs1, dest, csr),
                CSRRCI => Csrrci::csrrci(instruction.rs1, dest, csr),
                _ => return Err(Error::InvalidOpCode),
            };
        } else {
            let dest = ZeroOrRegister::from_u5(instruction.rd)
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = csrs.get(instruction.imm.as_u16() as usize);
        }
        Ok(())
    }
}

impl FloatS for u32 {
    #[inline(always)]
    fn floats(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error> {
        let f = match instruction.fid() {
            FADD_S => Fadd::fadd,
            FSUB_S => Fsub::fsub,
            FMUL_S => Fmul::fmul,
            FDIV_S => Fdiv::fdiv,
            FSQRT_S => Fsqrt::fsqrt,
            FSGNJ_S => Fsgnj::fsgnj,
            FSGNJN_S => Fsgnjn::fsgnjn,
            FSGNJNX_S => Fsgnjx::fsgnjx,
            FMIN_S => Fmin::fmin,
            FMAX_S => Fmax::fmax,
            // FCVT_W_S => Fcvtw::fcvtw,
            // FCVT_WU_S => Fcvtwu::fcvtwu,
            // FMV_X_W => Fmvxw::fmvxw,
            // FEQ_S => Feq::feq,
            // FLT_S => Flt::flt,
            // FLE_S => Fle::fle,
            // FCLASS_S => Fclass::fclass,
            // FCVT_S_W => Fcvtsw::fcvtsw,
            // FCVT_S_WU => Fcvtswu::fcvtswu,
            // FMV_W_X => Fmvwx::fmvwx,
            _ => return Err(Error::InvalidOpCode),
        };
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
}
