#![allow(dead_code)]
use crate::decode::{U10, U3};

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
    pub const ADD: U10 = 0b0000000_000;
    pub const SUB: U10 = 0b0100000_000;
    pub const SLL: U10 = 0b0000000_001;
    pub const SLT: U10 = 0b0000000_010;
    pub const SLTU: U10 = 0b0000000_011;
    pub const XOR: U10 = 0b0000000_100;
    pub const SRL: U10 = 0b0000000_101;
    pub const SRA: U10 = 0b0100000_101;
    pub const OR: U10 = 0b0000000_110;
    pub const AND: U10 = 0b0000000_111;
    pub const ADDW: U10 = 0b0000000_000;
    pub const SUBW: U10 = 0b0100000_000;
    pub const SLLW: U10 = 0b0000000_001;
    pub const SRLW: U10 = 0b0000000_101;
    pub const SRAW: U10 = 0b0100000_101;
    pub const ADDI: U3 = 0b000;
    pub const SLTI: U3 = 0b010;
    pub const SLTIU: U3 = 0b011;
    pub const XORI: U3 = 0b100;
    pub const ORI: U3 = 0b110;
    pub const ANDI: U3 = 0b111;
    pub const ADDIW:U3 = 0b000;
    pub const SLLIW: U10 = 0b0000000_001;
    pub const SRLIW: U10 = 0b0000000_101;
    pub const SRAIW: U10 = 0b0100000_101;
    pub const LB: U3 = 0b000;
    pub const LBU: U3 = 0b100;
    pub const LH: U3 = 0b001;
    pub const LHU: U3 = 0b101;
    pub const LW: U3 = 0b010;
    pub const LWU: U3 = 0b110;
    pub const LD: U3 = 0b011;
    pub const SLLI: U10 = 0b0000000_001;
    pub const SRLI: U10 = 0b0000000_101;
    pub const SRAI: U10 = 0b0100000_101;
    pub const SB: U3 = 0b000;
    pub const SH: U3 = 0b001;
    pub const SW: U3 = 0b010;
    pub const SD: U3 = 0b011;
    pub const BEQ: U3 = 0b000;
    pub const BNE: U3 = 0b001;
    pub const BLT: U3 = 0b100;
    pub const BGE: U3 = 0b101;
    pub const BLTU: U3 = 0b110;
    pub const BGEU: U3 = 0b111;
    // M extension
    pub const MUL: U10 = 0b0000001_000;
    pub const MULH: U10 = 0b0000001_001;
    pub const MULHSU: U10 = 0b0000001_010;
    pub const MULHU: U10 = 0b0000001_011;
    pub const DIV: U10 = 0b0000001_100;
    pub const DIVU: U10 = 0b0000001_101;
    pub const REM: U10 = 0b0000001_110;
    pub const REMU: U10 = 0b0000001_111;
}
