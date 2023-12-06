# Risky
A RISC-V emulator written in Rust

## Introduction

__N.B.__ The project is still in alpha stage and while all supported instructions have been tested independently, CI is missing tests against an entire program.

Risky starts as a toy project, built to get in touch with the RISC-V ISA and since I'm currently learning Rust I'm watering two plants with one hose.

It supports both 32bit and 64bit base instructions set. Support for some of RISC-V ratified extensions will come in the future.

## Installation

1. Clone the repo:
  `git clone https://githb.com/DontPanicO/risky.git`
2. Build the project:
  `cargo build` or `cargo build --release`

## Testing

Currently, Risky has tests for instruction decoding, execution, memory and register emulation running under ci.
Use `cargo test` to run them.

To test the emulator against an executable, you have to specify the path to the elf executable: `cargo run /path/to/riscv_executable`.

I use <https://github.com/litmus-tests/litmus-tests-riscv> to test execution locally (I'll add that in CI as soon as possible).
1. Clone the repo: `git clone https://github.com/litmus-tests/litmus-tests-riscv`.
2. If you want to compile for 32bit cd to `litmus-tests-riscv/elf-tests/basic/`, then edit the Makefile, replacing `CFLAGS += -march=rv64g -mabi=lp64d` with `CFLAGS += -march=rv32i -mabi=ilp32`.
3. Run `make clean` and `make all`.
4. Run the project against function_call* and loop* executables.
__N.B.__ Compilation requires riscv gnu toolchain.

You can also look at <https://github.com/riscv-software-src/riscv-tests>.
