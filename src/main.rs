pub(crate) mod decode;
pub(crate) mod elf;
pub(crate) mod error;
pub(crate) mod instruction_ids;
pub(crate) mod instructions;
pub(crate) mod mem;
pub(crate) mod num;
pub(crate) mod ops;
pub(crate) mod registers;

use crate::registers::ProgramCounter;
use ::elf::file::Class;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Missing executable path.");
        std::process::exit(1);
    }
    let path = &args[1];
    let mut memory = [0u8; 262140];
    let file = std::fs::read(path).unwrap();
    let elfdata = elf::load_elf_le(&file).unwrap();
    for sg in elfdata.segments().unwrap().iter() {
        let sg_data = elfdata.segment_data(&sg).unwrap();
        println!("{}, {}", sg.p_paddr, sg.p_memsz);
        mem::memw(sg_data, &mut memory, sg.p_paddr as usize).unwrap();
    }
    //...
    match elfdata.ehdr.class {
        Class::ELF32 => {
            let mut program_counter = elfdata.ehdr.e_entry as u32;
            let mut regs = registers::Registers::with_sp(256);
            let mut csrs = registers::CsrRegisters::new();
            loop {
                // fetch instruction (libmem::memr(4))
                let ins =
                    u32::from_le_bytes(mem::memr32(&memory, program_counter as usize).unwrap());
                // decode and execute instruction + increment the program counter
                step(ins, &mut regs, &mut program_counter, &mut memory, &mut csrs);
            }
        }
        Class::ELF64 => {
            let mut program_counter = elfdata.ehdr.e_entry;
            let mut regs = registers::Registers::with_sp(256);
            let mut csrs = registers::CsrRegisters::new();
            loop {
                // fetch instruction (libmem::memr(4))
                let ins =
                    u32::from_le_bytes(mem::memr32(&memory, program_counter as usize).unwrap());
                // decode and execute instruction + increment the program counter
                step(ins, &mut regs, &mut program_counter, &mut memory, &mut csrs);
            }
        }
    }
}

trait Step: Sized {
    fn step(
        encoded: u32,
        regs: &mut registers::Registers<Self>,
        pc: &mut Self,
        memory: &mut [u8],
        csrs: &mut registers::CsrRegisters<Self>,
    );
}

impl Step for u32 {
    #[inline(always)]
    fn step(
        encoded: u32,
        regs: &mut registers::Registers<Self>,
        pc: &mut Self,
        memory: &mut [u8],
        csrs: &mut registers::CsrRegisters<Self>,
    ) {
        match bit_extract(encoded, 0, 6) {
            0b0110111 => {
                let instruction = decode::U::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Lui::lui(instruction, regs).unwrap();
                pc.increment();
            }
            0b0010111 => {
                let instruction = decode::U::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Auipc::auipc(instruction, regs, *pc).unwrap();
                pc.increment();
            }
            0b1101111 => {
                let instruction = decode::J::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Jal::jal(instruction, regs, pc).unwrap();
            }
            0b1100111 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Jalr::jalr(instruction, regs, pc).unwrap();
            }
            0b1100011 => {
                let instruction = decode::B::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Branch::branch(instruction, regs, pc).unwrap();
            }
            0b0000011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Load::load(instruction, regs, memory).unwrap();
                pc.increment();
            }
            0b0100011 => {
                let instruction = decode::S::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Store::store(instruction, regs, memory).unwrap();
                pc.increment();
            }
            0b0010011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                if instruction.funct3.as_u8() == 0b001 || instruction.funct3.as_u8() == 0b101 {
                    instructions::ShiftI::shifti(instruction.into(), regs).unwrap()
                } else {
                    instructions::MathI::mathi(instruction, regs).unwrap()
                }
                pc.increment();
            }
            0b0110011 => {
                let instruction = decode::R::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Math::math(instruction, regs).unwrap();
                pc.increment();
            }
            0b0001111 => todo!("FENCE detected"),
            0b1110011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Csr::csr(instruction, regs, csrs).unwrap();
                pc.increment();
            }
            _ => panic!("Invalid OPCode"),
        }
    }
}

impl Step for u64 {
    #[inline(always)]
    fn step(
        encoded: u32,
        regs: &mut registers::Registers<Self>,
        pc: &mut Self,
        memory: &mut [u8],
        csrs: &mut registers::CsrRegisters<Self>,
    ) {
        match bit_extract(encoded, 0, 6) {
            0b0110111 => {
                let instruction = decode::U::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Lui::lui(instruction, regs).unwrap();
                pc.increment();
            }
            0b0010111 => {
                let instruction = decode::U::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Auipc::auipc(instruction, regs, *pc).unwrap();
                pc.increment();
            }
            0b1101111 => {
                let instruction = decode::J::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Jal::jal(instruction, regs, pc).unwrap();
            }
            0b1100111 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Jalr::jalr(instruction, regs, pc).unwrap();
            }
            0b1100011 => {
                let instruction = decode::B::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Branch::branch(instruction, regs, pc).unwrap();
            }
            0b0000011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Load::load(instruction, regs, memory).unwrap();
                pc.increment();
            }
            0b0100011 => {
                let instruction = decode::S::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Store::store(instruction, regs, memory).unwrap();
                pc.increment();
            }
            0b0010011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                if instruction.funct3.as_u8() == 0b001 || instruction.funct3.as_u8() == 0b101 {
                    instructions::ShiftI::shifti(instruction.into(), regs).unwrap()
                } else {
                    instructions::MathI::mathi(instruction, regs).unwrap()
                }
                pc.increment();
            }
            0b0110011 => {
                let instruction = decode::R::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Math::math(instruction, regs).unwrap();
                pc.increment();
            }
            0b0111011 => {
                let instruction = decode::R::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::MathW::mathw(instruction, regs).unwrap();
                pc.increment();
            }
            0b0011011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                if instruction.funct3.as_u8() == 0b000 {
                    instructions::MathIW::mathiw(instruction, regs).unwrap()
                } else {
                    instructions::ShiftIW::shiftiw(instruction.into(), regs).unwrap();
                }
                pc.increment();
            }
            0b0001111 => todo!("FENCE detected"),
            0b1110011 => {
                let instruction = decode::I::from_u32(encoded);
                println!("{:?}", instruction);
                instructions::Csr::csr(instruction, regs, csrs).unwrap();
                pc.increment();
            }
            _ => panic!("Invalid OPCode"),
        }
    }
}

#[inline(always)]
fn step<T>(
    encoded: u32,
    regs: &mut registers::Registers<T>,
    pc: &mut T,
    memory: &mut [u8],
    csrs: &mut registers::CsrRegisters<T>,
) where
    T: Copy + Step + instructions::BaseInstruction + registers::ProgramCounter + std::fmt::LowerHex,
{
    println!("{:#034b} - PC: {:#0x}", encoded, pc);
    T::step(encoded, regs, pc, memory, csrs);
}

#[inline(always)]
pub const fn bit_extract(src: u32, lo: u32, hi: u32) -> u32 {
    (src >> lo) & (2u32.pow(hi - lo + 1) - 1)
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;

    #[test]
    fn test_lui() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        let mut program_counter = 0u32;
        let instruction = 0b00000000000000000001_01100_0110111;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 0b1000000000000);
    }

    #[test]
    fn test_auipc() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        let mut program_counter = 4u32;
        let instruction = 0b00000000000000000001_01100_0010111;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 0b1000000000000 + 4);
    }

    #[test]
    fn test_jal() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        let mut program_counter = 4u32;
        let instruction = 0b0_0000000000_0_00000000_01100_1101111;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 8);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_jalr() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 12;
        let mut program_counter = 4u32;
        let instruction = 0b000000000000_01101_000_01100_1100111;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 8);
        assert_eq!(program_counter, 12 & !0b1);
    }

    #[test]
    fn test_load_byte() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        memory[32] = 255;
        let mut program_counter = 4u32;
        let instruction = 0b000000000000_01101_000_01100_0000011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12 as u8, 255);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_load_half() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        memory[32..34].copy_from_slice(&[255, 255]);
        let mut program_counter = 4u32;
        let instruction = 0b000000000000_01101_001_01100_0000011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12 as u16, u16::MAX);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_load_word() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        memory[32..36].copy_from_slice(&[255, 255, 0, 0]);
        let mut program_counter = 4u32;
        let instruction = 0b000000000000_01101_010_01100_0000011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, u32::from_le_bytes([255, 255, 0, 0]));
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_load_dword() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        memory[32..40].copy_from_slice(&[255, 255, 0, 0, 0, 0, 0, 0]);
        let mut program_counter = 4u64;
        let instruction = 0b000000000000_01101_011_01100_0000011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, u64::from_le_bytes([255, 255, 0, 0, 0, 0, 0, 0]));
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_store_byte() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 255;
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 4u32;
        let instruction = 0b0000000_01100_01101_000_00000_0100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let data = mem::read::<u8>(&memory, 32).unwrap();
        assert_eq!(data, 255);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_store_half() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = u32::from_le_bytes([255, 255, 0, 0]);
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 4u32;
        let instruction = 0b0000000_01100_01101_001_00000_0100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let data = mem::read::<mem::U16>(&memory, 32).unwrap().as_u16();
        assert_eq!(data, u16::MAX);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_store_word() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = u32::from_le_bytes([255, 255, 0, 0]);
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 4u32;
        let instruction = 0b0000000_01100_01101_010_00000_0100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let data = mem::read::<mem::U32>(&memory, 32).unwrap().as_u32();
        assert_eq!(data, u16::MAX as u32);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_store_dword() {
        let mut memory = [0u8; 64];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = u16::MAX as u64;
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 4u64;
        let instruction = 0b0000000_01100_01101_011_00000_0100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let data = mem::read::<mem::U64>(&memory, 32).unwrap().as_u64();
        assert_eq!(data, u16::MAX as u64);
        assert_eq!(program_counter, 8);
    }

    #[test]
    fn test_math_add() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        *regs.get_mut(registers::Register::X14) = 10;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_000_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 110);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_sub() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        *regs.get_mut(registers::Register::X14) = 10;
        let mut program_counter = 0u32;
        let instruction = 0b0100000_01110_01101_000_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 90);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_sll() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 1;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_001_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1 << 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_srl() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_101_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 >> 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_sra() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0100000_01110_01101_101_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, ((-1i32).wrapping_shr(4)) as u32);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_slt() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_010_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_sltu() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 2;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_011_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_xor() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_100_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 ^ 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_or() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_110_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 | 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_and() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01110_01101_111_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 & 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_math_mul() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -12i32 as u32;
        *regs.get_mut(registers::Register::X14) = 24;
        let mut program_counter = 0u32;
        let instruction = 0b0000001_01110_01101_000_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, (-12i32 * 24) as u32);
    }

    #[test]
    fn test_math_mulh() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -12i32 as u32;
        *regs.get_mut(registers::Register::X14) = 24;
        let mut program_counter = 0u32;
        let instruction = 0b0000001_01110_01101_001_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, (((-12i64 * 24i64) >> 32) as u64) as u32);
    }

    #[test]
    fn test_math_mulhu() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 12;
        *regs.get_mut(registers::Register::X14) = 6;
        let mut program_counter = 0u32;
        let instruction = 0b0000001_01110_01101_011_01100_0110011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 0);
    }

    #[test]
    fn test_mathi_addi() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_000_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 101);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathi_slti() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u32;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_010_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathi_sltiu() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u32;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_011_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 0);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathi_xori() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_100_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 ^ 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathi_ori() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_110_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 | 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathi_andi() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u32;
        let instruction = 0b000000000001_01101_111_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 & 1);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_slli() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 1;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_00011_01101_001_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1 << 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_srli() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_00011_01101_101_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 >> 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_srai() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u32;
        let mut program_counter = 0u32;
        let instruction = 0b0100000_00011_01101_101_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        println!("{:?}", regs);
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, ((-1i32).wrapping_shr(3)) as u32);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_slli_64() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 1;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_00011_01101_001_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1 << 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_srli_64() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_00011_01101_101_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 >> 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shifti_srai_64() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i32 as u64;
        let mut program_counter = 0u64;
        let instruction = 0b0100000_00011_01101_101_01100_0010011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        println!("{:?}", regs);
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, ((-1i64).wrapping_shr(3)) as u64);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_beq() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 32;
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_000_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_bne() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 32;
        *regs.get_mut(registers::Register::X13) = 64;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_001_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_blt() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 32;
        *regs.get_mut(registers::Register::X13) = 64;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_100_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_bltu() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 32;
        *regs.get_mut(registers::Register::X13) = 64;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_110_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_bge() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 64;
        *regs.get_mut(registers::Register::X13) = 64;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_101_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_branch_bgeu() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X12) = 65;
        *regs.get_mut(registers::Register::X13) = 64;
        let mut program_counter = 0u32;
        let instruction = 0b0000000_01101_01100_111_00100_1100011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathw_addw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        *regs.get_mut(registers::Register::X14) = 10;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_01110_01101_000_01100_0111011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 110);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathw_subw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        *regs.get_mut(registers::Register::X14) = 10;
        let mut program_counter = 0u64;
        let instruction = 0b0100000_01110_01101_000_01100_0111011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 90);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathw_sllw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 1;
        *regs.get_mut(registers::Register::X14) = 4;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_01110_01101_001_01100_0111011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1 << 4);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathw_srlw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        *regs.get_mut(registers::Register::X14) = 3;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_01110_01101_101_01100_0111011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 >> 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathw_sraw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i64 as u64;
        *regs.get_mut(registers::Register::X14) = 3;
        let mut program_counter = 0u64;
        let instruction = 0b0100000_01110_01101_101_01100_0111011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12 as u32, ((-1i64).wrapping_shr(3) as u64 as u32));
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_mathiw_addiw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 100;
        let mut program_counter = 0u64;
        let instruction = 0b000000000011_01101_000_01100_0011011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 103);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shiftiw_slliw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 1;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_00011_01101_001_01100_0011011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 1 << 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shiftiw_srliw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = 32;
        let mut program_counter = 0u64;
        let instruction = 0b0000000_00011_01101_101_01100_0011011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12, 32 >> 3);
        assert_eq!(program_counter, 4);
    }

    #[test]
    fn test_shiftiw_sraiw() {
        let mut memory = [0u8; 0];
        let mut regs = registers::Registers::default();
        let mut csrs = registers::CsrRegisters::new();
        *regs.get_mut(registers::Register::X13) = -1i64 as u64;
        let mut program_counter = 0u64;
        let instruction = 0b0100000_00011_01101_101_01100_0011011;
        step(
            instruction,
            &mut regs,
            &mut program_counter,
            &mut memory,
            &mut csrs,
        );
        let r12 = regs.get(registers::Register::X12);
        assert_eq!(r12 as u32, (-1i64).wrapping_shr(3) as u64 as u32);
        assert_eq!(program_counter, 4);
    }
}
