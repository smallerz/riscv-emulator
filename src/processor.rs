/// Processor
/// The processor is currently only designed to support the
/// RV32I variant of the ISA, meaning registers are 32 bits in size.

// TODO:
// I'm using unwrap() during prototyping, but these will need
// to be replaced once it's clearer how errors should be handled.
// Most fields on instructions return an Option, so I'd like to be
// able to use error propagation (e.g. instr.funct3()?...).

use crate::alu::ALU;
use crate::instruction::{
    Instruction,
    InstructionFormat::*,
};

const IALIGN: u32 = 32;
const XLEN: u32 = 32;

//const HALFWORD: u32 = 16;
const WORD: u32 = 32;
//const DOUBLEWORD: u32 = 64;
//const QUADWORD: u32 = 128;

#[derive(Debug)]
pub struct Processor {
    /// Arithmetic Logic Unit (ALU)
    /// Responsible for performing arithmetic operations.
    pub alu: ALU,

    /// Program Counter (PC)
    /// Contains the address of the instruction being executed.
    pub pc: u32,

    /// `x` Registers
    /// Integer registers that are part of the base ISA,
    /// comprised of a zero register and 31 general-purpose
    /// registers.
    pub reg_x: [u32; 32],
}

impl Processor {
    /// Creates a new processor.
    pub fn new() -> Self {
        Self {
            alu: ALU {},
            pc: 0x00,
            reg_x: [0x00; XLEN as usize],
        }
    }

    /// Execute an instruction.
    pub fn execute(&mut self, instr: &Instruction) -> () {
        match instr.format() {
            B => {
                todo!();
            },

            I => {
                match instr.opcode() {
                    // Load Instructions
                    0x03 => {
                        match instr.funct3().unwrap() {
                            // lb rd, rs1, imm
                            0x00 => { todo!(); },
                            // lh rd, rs1, imm
                            0x01 => { todo!(); },
                            // lw rd, rs1, imm
                            0x02 => { todo!(); },
                            // lbu rd, rs1, imm
                            0x04 => { todo!(); },
                            // lhu rd, rs1, imm
                            0x05 => { todo!(); },
                            // Invalid instruction
                            _ => todo!(),
                        }
                    },

                    // fence/fence.i
                    0x0f => {
                        match instr.funct3().unwrap() {
                            // fence
                            0x00 => { todo!(); },
                            // fence.i
                            0x01 => { todo!(); },
                            // Invalid instruction
                            _ => todo!(),
                        }
                    },

                    // Shift & Logical Immediate Instructions
                    0x13 => {
                        match instr.funct3().unwrap() {
                            // addi rd, rs1, imm
                            0x00 => { todo!(); },
                            // slli rd, rs1, imm
                            0x01 => { todo!(); },
                            // slti rd, rs1, imm
                            0x02 => { todo!(); },
                            // sltiu rd, rs1, imm
                            0x03 => { todo!(); },
                            // xori rd, rs1, imm
                            0x04 => { todo!(); },
                            // srli/srai
                            0x05 => {
                                match (instr.imm().unwrap() >> 0x05 & 0x7f) as u8 {
                                    // srli rd, rs1, imm
                                    0x00 => { todo!(); },
                                    // srai rd, rs1, imm
                                    0x20 => { todo!(); },
                                    // Invalid instruction
                                    _ => todo!(),
                                }
                            },
                            // ori rd, rs1, imm
                            0x06 => { todo!(); },
                            // andi rd, rs1, imm
                            0x07 => { todo!(); },
                            // Invalid instruction
                            _ => todo!(),
                        }
                    },

                    // jalr
                    0x67 => { todo!(); },

                    // CSR*, ebreak, ecall
                    0x73 => {
                        match instr.funct3().unwrap() {
                            // ebreak/ecall
                            0x00 => {
                                // TODO:
                                // Two arms to add here:
                                // 1. ebreak
                                // 2. ecall
                                // The arms are based on some of the bits in the imm field,
                                // but I'm not sure how that value is determined yet.
                                todo!();
                            },
                            // CSRRW
                            0x01 => { todo!(); },
                            // CSRRS
                            0x02 => { todo!(); },
                            // CSRRC
                            0x03 => { todo!(); },
                            // CSRRWI
                            0x05 => { todo!(); },
                            // CSRRSI
                            0x06 => { todo!(); },
                            // CSRRCI
                            0x07 => { todo!(); },
                            // Invalid instruction
                            _ => todo!(),
                        }
                    },

                    // Invalid instruction
                    _ => todo!(),
                }
            },

            // jal rd, imm
            J => {
                // TO DO:
                // Make sure that your implementation factors in that the return address
                // can be the zero register.

                // The base ISA has IALIGN=32, meaning that instructions must
                // be aligned on a four-byte boundary in memory. An instruction-address-misaligned exception is
                // generated on a taken branch or unconditional jump if the target address is not IALIGN-bit aligned.
                // This exception is reported on the branch or jump instruction, not on the target instruction. No
                // instruction-address-misaligned exception is generated for a conditional branch that is not taken.
                if instr.imm().unwrap() % (IALIGN / 8) as i64 != 0 {
                    todo!();
                }

                // TO DO:
                // I'm not sure what should happen if the program counter overflows
                // when we increment it below. At the moment, it wraps around.

                // Set the return address to the current value in the program counter
                // plus the size of an instruction in bytes (in the case of RV32I, 4 bytes).
                self.reg_x[instr.rd().unwrap() as usize] = self.pc.wrapping_add(WORD / 8);

                // TO DO:
                // Use the imm field as an offset relative to the program counter to
                // jump to the address in memory of the next instruction to execute.
                todo!();
            },

            R => {
                match instr.funct3().unwrap() {

                    0x00 => {
                        match instr.funct7().unwrap() {

                            // add rd, rs1, rs2
                            0x00 => {
                                self.reg_x[instr.rd().unwrap() as usize] =
                                    self.alu.add(
                                        self.reg_x[instr.rs1().unwrap() as usize],
                                        self.reg_x[instr.rs2().unwrap() as usize]
                                    );
                            },
                            
                            // sub rd, rs1, rs2
                            //0x20 => {},

                            _ => todo!(),
                        }
                    },

                    //0x01 => ALU::sll(&instr),
                    // ...
                    _ => todo!(),
                }
            },

            S => {
                todo!();
            },
        }
    }

    /// Fetches and returns the next instruction to 
    /// execute from memory.
    pub fn fetch(&self) -> Instruction {
        todo!();
    }
}
