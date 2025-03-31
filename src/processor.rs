/// Processor
/// The processor is currently only designed to support the
/// RV32I variant of the ISA, meaning registers are 32 bits in size.

// TODO:
// I'm using unwrap() during prototyping, but these will need
// to be replaced once it's clearer how errors should be handled.
// Most fields on instructions return an Option, so I'd like to be
// able to use error propagation (e.g. instr.funct3()?...).

use crate::alu::Alu;

use crate::decode::Decoder;

use crate::instruction::{
    Instruction,
    InstructionFormat::*,
};

use crate::op::{
    Op,
    Op::*,
};

use crate::register::{
    AccessLevel,
    RegistersX,
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
    pub alu: Alu,

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
            alu: Alu::new(),
            pc: 0x00,
            reg_x,
        }
    }

    /// Executes an instruction.
    pub fn execute(&mut self, instr: &Instruction) {
        match instr.format() {
            B => self.exec_instr_b(instr),
            I => self.exec_instr_i(instr),
            J => self.exec_instr_j(instr),
            R => self.exec_instr_r(instr),
            S => self.exec_instr_s(instr),
            U => self.exec_instr_u(instr),
        }
    }
    
    /// Executes a B-type instruction.
    #[inline]
    fn exec_instr_b(&mut self, instr: &Instruction) {
        match Decoder::decode(instr) {
            op @ Some(
                BranchEqual
                | BranchGreaterThanOrEqualTo
                | BranchGreaterThanOrEqualToUnsigned
                | BranchLessThan
                | BranchLessThanUnsigned
                | BranchNotEqual
            ) => {
                if let 1 = self.alu.run(
                    &op.unwrap(),
                    self.reg_x.read(
                        instr.rs1().unwrap(),
                    ) as i32, 
                    self.reg_x.read(
                        instr.rs2().unwrap(),
                    ) as i32,
                ) {
                    // TODO:
                    // The conditional branch instructions will generate an 
                    // instruction-address-misaligned exception if the
                    // target address is not aligned to a four-byte boundary
                    // and the branch condition evaluates to true. If the
                    // branch condition evaluates to false, the 
                    // instruction-address-misaligned exception will not be raised.
        
                    // NOTE:
                    // Instruction-address-misaligned exceptions are not possible
                    // on machines that support extensions with 16-bit aligned 
                    // instructions, such as the compressed instruction-set
                    // extension, C.
        
                    self.pc = self.pc.wrapping_add_signed(
                        instr.imm().unwrap(),
                    );
                }
            },

            _ => self.handle_illegal_instr(instr),
        }
    }
    
    /// Executes an I-type instruction.
    #[inline]
    fn exec_instr_i(&mut self, instr: &Instruction) {            
        match Decoder::decode(instr) {  
            op @ Some(
                ArithmeticAddImmediate 
                | LogicalAndImmediate
                | LogicalExclusiveOrImmediate
                | LogicalOrImmediate
                | ShiftLeftLogicalImmediate
                | ShiftRightArithmeticImmediate
                | ShiftRightLogicalImmediate
            ) => {
                self.reg_x.write(
                    instr.rd().unwrap(),
                    self.alu.run(
                        &op.unwrap(), 
                        self.reg_x.read(
                            instr.rs1().unwrap(),
                        ) as i32,
                        instr.imm().unwrap(),
                    ) as u32,
                );
            },

            op @ Some(
                JumpAndLinkRegister,
            ) => {
                self.exec_jump(
                    op.unwrap(),
                    instr,
                );
            }, 

            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes a J-type instruction.
    #[inline]
    fn exec_instr_j(&mut self, instr: &Instruction) {
        match Decoder::decode(instr) {
            op @ Some(
                JumpAndLink,
            ) => {
                self.exec_jump(
                    op.unwrap(),
                    instr,
                );
            },

            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes an R-type instruction.
    #[inline]
    fn exec_instr_r(&mut self, instr: &Instruction) {
        match Decoder::decode(instr) {
            op @ Some(
                ArithmeticAdd
                | ArithmeticSub
                | LogicalAnd
                | LogicalExclusiveOr
                | LogicalOr
                | ShiftLeftLogical
                | ShiftRightArithmetic
                | ShiftRightLogical
            ) => {
                self.reg_x.write(
                    instr.rd().unwrap(),
                    self.alu.run(
                        &op.unwrap(), 
                        self.reg_x.read(
                            instr.rs1().unwrap(),
                        ) as i32,
                        self.reg_x.read(
                            instr.rs2().unwrap(),
                        ) as i32,
                    ) as u32,
                );
            },

            _ => self.handle_illegal_instr(instr),
        }
    }

    /// Executes an S-type instruction.
    #[inline]
    fn exec_instr_s(&mut self, instr: &Instruction) {
        todo!("exec_instr_s not yet implemented.");
    }

    /// Executes a U-type instruction.
    #[inline]
    fn exec_instr_u(&mut self, instr: &Instruction) {
        match Decoder::decode(instr) {
            op @ Some(
                AddUpperImmediateProgramCounter
                | LoadUpperImmediate
            ) => {
                let mut addr: u32 = self.alu.run(
                    &ShiftLeftLogicalImmediate,
                    instr.imm().unwrap(),
                    12,
                ) as u32;
        
                if let AddUpperImmediateProgramCounter = op.unwrap() {
                    addr = self.alu.run(
                        &ArithmeticAddImmediate,
                        addr as i32,
                        self.pc as i32,
                    ) as u32;
                }
        
                self.reg_x.write(
                    instr.rd().unwrap(),
                    addr,
                );
            },

            _ => self.handle_illegal_instr(instr),
        }
    }

    fn exec_jump(&mut self, op: Op, instr: &Instruction) {
        // Write the return address to the destination register.
        self.reg_x.write(
            instr.rd().unwrap(),
            self.pc + 0x04,
        );

        // Calculate the branch target and set the program counter.
        self.pc = match op {
            // target = pc + imm
            JumpAndLink => {
                self.pc.wrapping_add_signed(
                    instr.imm().unwrap(),
                )
            },
            
            // target = (rs1 + imm) & !1
            JumpAndLinkRegister => {
                (
                    self.reg_x
                        .read(
                            instr.rs1().unwrap(),
                        )
                        .wrapping_add_signed(
                            instr.imm().unwrap(),
                        )
                ) & !0x01
            },

            _ => self.pc
        };
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
