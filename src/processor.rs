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
            Some(ArithmeticAdd)                 => self.exec_add(instr),
            Some(ArithmeticAddImmediate)        => self.exec_addi(instr),
            Some(ArithmeticSub)                 => self.exec_sub(instr),
            Some(LoadUpperImmediate)            => todo!(),
            Some(LogicalAnd)                    => self.exec_and(instr),
            Some(LogicalAndImmediate)           => self.exec_andi(instr),
            Some(LogicalExclusiveOr)            => self.exec_xor(instr),
            Some(LogicalExclusiveOrImmediate)   => self.exec_xori(instr),
            Some(LogicalOr)                     => self.exec_or(instr),
            Some(LogicalOrImmediate)            => self.exec_ori(instr),
            _                                   => self.handle_illegal_instr(instr),
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

    /// Executes an `add` instruction.
    fn exec_add(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            .wrapping_add_signed(self.reg_x.read(instr.rs2().unwrap()) as i32);

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
    }

    /// Executes an `addi` instruction.
    fn exec_addi(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            .wrapping_add_signed(instr.imm().unwrap());

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
    }

    fn exec_and(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            & self.reg_x.read(instr.rs2().unwrap());

        self.reg_x.write(
            instr.rd().unwrap(), 
            result,
        );
    }

    fn exec_andi(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            & instr.imm().unwrap() as u32;

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
    }

    fn exec_or(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            | self.reg_x.read(instr.rs2().unwrap());

        self.reg_x.write(
            instr.rd().unwrap(), 
            result,
        );
    }

    fn exec_ori(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            | instr.imm().unwrap() as u32;

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
    }

    fn exec_xor(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            ^ self.reg_x.read(instr.rs2().unwrap());

        self.reg_x.write(
            instr.rd().unwrap(), 
            result,
        );
    }

    fn exec_xori(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            ^ instr.imm().unwrap() as u32;

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
    }

    fn exec_sub(&mut self, instr: &Instruction) {
        let result = self.reg_x.read(instr.rs1().unwrap())
            .wrapping_sub(
                self.reg_x.read(instr.rs2().unwrap())
            );

        self.reg_x.write(
            instr.rd().unwrap(),
            result,
        );
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
