pub(crate) mod decode;
pub(crate) mod elf;
pub(crate) mod error;
pub(crate) mod instructions;
pub(crate) mod mem;
pub(crate) mod registers;

fn main() {
    let mut memory = [0u8; 262140];
    let mut regs = registers::Registers::default();
    let file = std::fs::read(
        "/home/andreatedeschi/Public/tests/riscv/litmus-tests-riscv/elf-tests/basic/build/function_call_1-O0",
    )
    .unwrap();
    let elfdata = elf::load_elf_le(&file).unwrap();
    let mut program_counter: u32 = elfdata.ehdr.e_entry as u32;
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
fn step(encoded: u32, regs: &mut registers::Registers<u32>, pc: &mut u32, memory: &mut [u8]) {
    println!("{:#034b} - PC: {:#0x}", encoded, pc);
    match bit_extract(encoded, 0, 6) {
        0b0110111 => {
            let instruction = decode::U::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_lui(instruction, regs).unwrap();
            *pc += 4;
        }
        0b0010111 => {
            let instruction = decode::U::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_auipc(instruction, regs, *pc).unwrap();
            *pc += 4;
        }
        0b1101111 => {
            let instruction = decode::J::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_jal(instruction, regs, pc).unwrap();
        }
        0b1100111 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_jalr(instruction, regs, pc).unwrap();
        }
        0b1100011 => {
            let instruction = decode::B::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_branch(instruction, regs, pc).unwrap();
        }
        0b0000011 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_load(instruction, regs, memory).unwrap();
            *pc += 4;
        }
        0b0100011 => {
            let instruction = decode::S::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_store(instruction, regs, memory).unwrap();
            *pc += 4;
        }
        0b0010011 => {
            let instruction = decode::I::from_u32(encoded);
            println!("{:?}", instruction);
            if instruction.funct3.as_u8() == 0b001 || instruction.funct3.as_u8() == 0b101 {
                instructions::execute_shifti(instruction.into(), regs).unwrap()
            } else {
                instructions::execute_mathi(instruction, regs).unwrap()
            }
            *pc += 4;
        }
        0b0110011 => {
            let instruction = decode::R::from_u32(encoded);
            println!("{:?}", instruction);
            instructions::execute_math(instruction, regs).unwrap();
            *pc += 4;
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
