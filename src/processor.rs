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
            reg_x: [0x00; 32],
        }
    }

    /// Execute an instruction.
    pub fn execute(&mut self, instr: &Instruction) -> () {
        match instr.format() {
            B => {
                todo!();
            },

            I => {
                todo!();
            },

            // jal rd, imm
            J => {
                // TO DO:
                // Make sure that your implementation factors in that the return address
                // can be the zero register.

                // TO DO:
                // I'm not sure what should happen if the program counter overflows
                // when we increment it below. At the moment, it wraps around.

                // Set the return address to the current value in the program counter
                // plus the size of an address in bytes (in the case of RV32I, 4 bytes).
                self.reg_x[instr.rd().unwrap() as usize] = self.pc.wrapping_add(0x04);

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
