pub(crate) mod decode;
pub(crate) mod elf;
pub(crate) mod error;
pub(crate) mod instruction_ids;
pub(crate) mod instructions;
pub(crate) mod mem;
pub(crate) mod num;
pub(crate) mod ops;
pub(crate) mod registers;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Missing executable path.");
        std::process::exit(1);
    }
    let path = &args[1];
    let mut memory = [0u8; 262140];
    let mut regs = registers::Registers::with_sp(256);
    let file = std::fs::read(path).unwrap();
    let elfdata = elf::load_elf_le(&file).unwrap();
    let mut program_counter = elfdata.ehdr.e_entry;
    for sg in elfdata.segments().unwrap().iter() {
        let sg_data = elfdata.segment_data(&sg).unwrap();
        println!("{}, {}", sg.p_paddr, sg.p_memsz);
        mem::memw(sg_data, &mut memory, sg.p_paddr as usize).unwrap();
    }
    //...
    loop {
        // fetch instruction (libmem::memr(4)), increase pc of 4
        let ins = u32::from_le_bytes(mem::memr32(&memory, program_counter as usize).unwrap());
        // decode and execute instruction
        step(ins, &mut regs, &mut program_counter, &mut memory);
        // increment the program counter
    }
}

#[inline(always)]
fn step<T>(encoded: u32, regs: &mut registers::Registers<T>, pc: &mut T, memory: &mut [u8])
where
    T: Copy + instructions::BaseInstruction + registers::ProgramCounter + std::fmt::LowerHex,
{
    println!("{:#034b} - PC: {:#0x}", encoded, pc);
    match bit_extract(encoded, 0, 6) {
        0b0110111 => {
            let instruction = decode::U::from_u32(encoded);
            println!("{:?}", instruction);
            T::lui(instruction, regs).unwrap();
            pc.increment();
        }
        0b0010111 => {
            let instruction = decode::U::from_u32(encoded);
            println!("{:?}", instruction);
            T::auipc(instruction, regs, *pc).unwrap();
            pc.increment();
        }
        0b1101111 => {
            let instruction = decode::J::from_u32(encoded);
            println!("{:?}", instruction);
            T::jal(instruction, regs, pc).unwrap();
        }
        0b1100111 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            T::jalr(instruction, regs, pc).unwrap();
        }
        0b1100011 => {
            let instruction = decode::B::from_u32(encoded);
            println!("{:?}", instruction);
            T::branch(instruction, regs, pc).unwrap();
        }
        0b0000011 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            T::load(instruction, regs, memory).unwrap();
            pc.increment();
        }
        0b0100011 => {
            let instruction = decode::S::from_u32(encoded);
            println!("{:?}", instruction);
            T::store(instruction, regs, memory).unwrap();
            pc.increment();
        }
        0b0010011 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            if instruction.funct3.as_u8() == 0b001 || instruction.funct3.as_u8() == 0b101 {
                T::shifti(instruction.into(), regs).unwrap()
            } else {
                T::mathi(instruction, regs).unwrap()
            }
            pc.increment();
        }
        0b0110011 => {
            let instruction = decode::R::from_u32(encoded);
            println!("{:?}", instruction);
            T::math(instruction, regs).unwrap();
            pc.increment();
        }
        0b0111011 => {
            let instruction = decode::R::from_u32(encoded);
            println!("{:?}", instruction);
            T::mathw(instruction, regs).unwrap();
            pc.increment();
        }
        0b0011011 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            if instruction.funct3.as_u8() == 0b000 {
                T::mathiw(instruction, regs).unwrap()
            } else {
                T::shiftiw(instruction.into(), regs).unwrap();
            }
            pc.increment();
        }
        0b0001111 => todo!("FENCE detected"),
        0b1110011 => todo!("SYSTEM call"),
        _ => panic!("Invalid OPCode"),
    }
}

#[inline(always)]
pub const fn bit_extract(src: u32, lo: u32, hi: u32) -> u32 {
    (src >> lo) & (2u32.pow(hi - lo + 1) - 1)
}
