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
use crate::register::{AccessLevel, RegistersX};

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
    pub reg_x: RegistersX,
}

impl Processor {
    /// Creates a new processor.
    pub fn new() -> Self {
        let mut reg_x = RegistersX::new();

        // All general-purpose registers besides the zero register will 
        // be read/write.
        for i in 1 .. reg_x.len() - 1 {
            reg_x.set_access_level(i, AccessLevel::ReadWrite);
        }

        Self {
            alu: ALU {},
            pc: 0x00,
            reg_x,
        }
    }

    /// Executes an instruction.
    pub fn execute(&mut self, instr: &Instruction) -> () {
        match instr.format() {
            // Branch Instructions
            B => self.execute_instr_b(instr),

            // Register-Immediate Instructions
            I => self.execute_instr_i(instr),

            // Jump Instructions
            J => self.execute_instr_j(instr),

            // Register-Register Instructions
            R => self.execute_instr_r(instr),

            // Store Instructions
            S => self.execute_instr_s(instr),

            // Upper-Immediate Instructions
            U => self.execute_instr_u(instr),
        }
    }

    /// Executes a B-type instruction.
    #[inline]
    fn execute_instr_b(&self, instr: &Instruction) {
        match instr.opcode() {
            0x63 => {
                match instr.funct3().unwrap() {
                    // beq rs1, rs2, label
                    0x00 => { todo!(); },
                    // bne rs1, rs2, label
                    0x01 => { todo!(); },
                    // blt rs1, rs2, label
                    0x04 => { todo!(); },
                    // bge rs1, rs2, label
                    0x05 => { todo!(); },
                    // bltu rs1, rs2, label
                    0x06 => { todo!(); },
                    // bgeu rs1, rs2, label
                    0x07 => { todo!(); },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },
            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes an I-type instruction.
    #[inline]
    fn execute_instr_i(&mut self, instr: &Instruction) {
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
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },

            // fence/fence.i
            0x0f => {
                match instr.funct3().unwrap() {
                    // fence
                    0x00 => { todo!(); },
                    // fence.i
                    0x01 => { todo!(); },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },

            // Shift & Logical Immediate Instructions
            0x13 => {
                match instr.funct3().unwrap() {
                    // addi rd, rs1, imm
                    0x00 => {
                        let result = self.alu.add(
                            self.reg_x.read(
                                instr.rs1().unwrap() as usize) as i32,
                            instr.imm().unwrap() as i32
                        );

                        self.reg_x.write(
                            instr.rd().unwrap() as usize,
                            result as u32
                        );
                    },
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
                            // Illegal instruction
                            _ => self.handle_illegal_instr(instr),
                        }
                    },
                    // ori rd, rs1, imm
                    0x06 => { todo!(); },
                    // andi rd, rs1, imm
                    0x07 => { todo!(); },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },

            // Jump-and-link Immediate
            0x67 => {
                match instr.funct3().unwrap() {
                    // jalr rd, imm(rs1)
                    0x00 => {
                        // 1. imm + rs1
                        // 2. Set 
                        todo!();
                    },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },

            // CSR*, ebreak, ecall
            0x73 => {
                match instr.funct3().unwrap() {
                    // ebreak/ecall
                    0x00 => {
                        match instr.imm().unwrap() {
                            // ecall rd, funct3, rs1, imm
                            0x00 => todo!(),
                            // ebreak rd, funct3, rs1, imm
                            0x01 => todo!(),
                            // Illegal instruction
                            _ => self.handle_illegal_instr(instr),
                        }
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
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },

            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes a J-type instruction.
    #[inline]
    fn execute_instr_j(&mut self, instr: &Instruction) {
        match instr.opcode() {
            // jal rd, imm
            0x6f => {
                // TO DO:
                // Make sure that your implementation factors in that the 
                // return address can be the zero register.

                // The base ISA has IALIGN=32, meaning that instructions must
                // be aligned on a four-byte boundary in memory. An 
                // instruction-address-misaligned exception is
                // generated on a taken branch or unconditional jump if the 
                // target address is not IALIGN-bit aligned.
                // This exception is reported on the branch or jump
                // instruction, not on the target instruction. No
                // instruction-address-misaligned exception is generated for a 
                // conditional branch that is not taken.
                if instr.imm().unwrap() % (IALIGN / 8) as i32 != 0 {
                    todo!();
                }

                // TO DO:
                // I'm not sure what should happen if the program counter
                // overflows when we increment it below. At the moment, it 
                // wraps around.

                // Set the return address to the current value in the program 
                // counter plus the size of an instruction in bytes (in the 
                // case of RV32I, 4 bytes).
                self.reg_x.write(
                    instr.rd().unwrap() as usize,
                    self.pc.wrapping_add(WORD / 8)
                );

                // TO DO:
                // Use the imm field as an offset relative to the program
                // counter to jump to the address in memory of the next instruction to execute.
                todo!();
            }
            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes an R-type instruction.
    #[inline]
    fn execute_instr_r(&mut self, instr: &Instruction) {
        match instr.opcode() {
            0x33 => {
                match instr.funct3().unwrap() {
                    // add/sub
                    0x00 => {
                        match instr.funct7().unwrap() {
                            // add rd, rs1, rs2
                            0x00 => {
                                let result = self.alu.add(
                                    self.reg_x.read(
                                        instr.rs1().unwrap() as usize)
                                        as i32,
                                    self.reg_x.read(
                                        instr.rs2().unwrap() as usize)
                                        as i32,
                                );
        
                                self.reg_x.write(
                                    instr.rd().unwrap() as usize,
                                    result as u32
                                );
                            },
                            // sub rd, rs1, rs2
                            0x20 => { todo!(); },
                            // Illegal instruction
                            _ => self.handle_illegal_instr(instr),
                        }
                    },
                    // sll rd, rs1, rs2
                    0x01 => { todo!(); },
                    // slt rd, rs1, rs2
                    0x02 => { todo!(); },
                    // sltu rd, rs1, rs2
                    0x03 => { todo!(); },
                    // xor rd, rs1, rs2
                    0x04 => { todo!(); },
                    // srl/sra
                    0x05 => {
                        match instr.funct7().unwrap() {
                            // srl rd, rs1, rs2
                            0x00 => { todo!(); },
                            // sra rd, rs1, rs2
                            0x20 => { todo!(); },
                            // Illegal instruction
                            _ => self.handle_illegal_instr(instr),
                        }
                    },
                    // or rd, rs1, rs2
                    0x06 => { todo!(); },
                    // and rd, rs1, rs2
                    0x07 => { todo!(); },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            },
            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes an S-type instruction.
    #[inline]
    fn execute_instr_s(&self, instr: &Instruction) {
        match instr.opcode() {
            0x23 => {
                match instr.funct3().unwrap() {
                    // sb rs1, imm(rs2)
                    0x00 => { todo!(); },
                    // sh rs1, imm(rs2)
                    0x01 => { todo!(); },
                    // sw rs1, imm(rs2)
                    0x02 => { todo!(); },
                    // Illegal instruction
                    _ => self.handle_illegal_instr(instr),
                }
            }
            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes a U-type instruction.
    #[inline]
    fn execute_instr_u(&self, instr: &Instruction) {
        match instr.opcode() {
            // lui rd, imm
            0x37 => { todo!(); },
            // Illegal instruction
            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Fetches and returns the next instruction to execute from memory.
    pub fn fetch(&self) -> Instruction {
        todo!();
    }

    /// Handles an illegal instruction by raising an illegal instruction
    /// exception.
    #[cold]
    fn handle_illegal_instr(&self, instr: &Instruction) {
        todo!();
    }
}
