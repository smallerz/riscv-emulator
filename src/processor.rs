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
use crate::instruction::Instruction;
use crate::op::{ Op, Op::* };
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
        match Decoder::decode(instr) {
            // Instructions that perform a conditional branch.
            op @ Some(
                BranchEqual
                | BranchGreaterThanOrEqualTo
                | BranchGreaterThanOrEqualToUnsigned
                | BranchLessThan
                | BranchLessThanUnsigned
                | BranchNotEqual
            ) => {
                self.exec_alu_op_b(
                    &op.unwrap(),
                    instr,
                )
            },

            // Instructions that perform register-immediate operations.
            op @ Some(
                ArithmeticAddImmediate 
                | LogicalAndImmediate
                | LogicalExclusiveOrImmediate
                | LogicalOrImmediate
                | ShiftLeftLogicalImmediate
                | ShiftRightArithmeticImmediate
                | ShiftRightLogicalImmediate
            ) => {
                self.exec_alu_op_i(
                    &op.unwrap(), 
                    instr,
                );
            },

            // Instructions that perform an unconditional branch.
            op @ Some(
                JumpAndLink
                | JumpAndLinkRegister
            ) => {
                self.exec_jump(
                    &op.unwrap(), 
                    instr
                );
            },

            // Instructions that perform register-register operations.
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
                self.exec_alu_op_r(
                    &op.unwrap(), 
                    instr,
                );
            },

            op @ Some(
                AddUpperImmediateProgramCounter
                | LoadUpperImmediate
            ) => {
                self.exec_instr_u(
                    &op.unwrap(),
                    instr,
                );
            },

            _ => self.handle_illegal_instr(instr),
        }
    }
    
    fn exec_alu_op_b(&mut self, op: &Op, instr: &Instruction) {
        if let 1 = self.alu.run(
            op, 
            self.reg_x.read(instr.rs1().unwrap()) as i32, 
            self.reg_x.read(instr.rs2().unwrap()) as i32,
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

            self.pc = self.pc.wrapping_add_signed(instr.imm().unwrap());
        }
    }
    
    fn exec_alu_op_r(&mut self, op: &Op, instr: &Instruction) {
        self.reg_x.write(
            instr.rd().unwrap(),
            self.alu.run(
                op, 
                self.reg_x.read(instr.rs1().unwrap()) as i32,
                self.reg_x.read(instr.rs2().unwrap()) as i32,
            ) as u32
        );
    }
    
    fn exec_alu_op_i(&mut self, op: &Op, instr: &Instruction) {
        self.reg_x.write(
            instr.rd().unwrap(),
            self.alu.run(
                op, 
                self.reg_x.read(instr.rs1().unwrap()) as i32,
                instr.imm().unwrap(),
            ) as u32
        );
    }

    /// Executes a U-type instruction.
    #[inline]
    fn exec_instr_u(&mut self, op: &Op, instr: &Instruction)
    {
        let mut addr: u32 = self.alu.run(
            &ShiftLeftLogicalImmediate,
            instr.imm().unwrap(),
            12,
        ) as u32;

        if let &AddUpperImmediateProgramCounter = op {
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
    }

    fn exec_jump(&mut self, op: &Op, instr: &Instruction) {
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
                (self.reg_x.read(instr.rs1().unwrap())
                    .wrapping_add_signed(instr.imm().unwrap()))
                    & !0x01
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
