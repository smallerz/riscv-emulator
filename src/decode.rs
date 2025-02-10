use crate::instruction::{
    Instruction,
    InstructionFormat::*,
};
use crate::op::{
    Op,
    Op::*,
};

/// Decodes an instruction into an operation.
pub struct Decoder;

impl Decoder {
    pub fn decode(instr: &Instruction) -> Option<Op> {
        match instr.format() {
            B => Decoder::decode_instr_b(instr),
            I => Decoder::decode_instr_i(instr),
            J => Decoder::decode_instr_j(instr),
            R => Decoder::decode_instr_r(instr),
            S => Decoder::decode_instr_s(instr),
            U => Decoder::decode_instr_u(instr),
        }
    }

    /// Decodes a B-type instruction.
    #[inline]
    fn decode_instr_b(instr: &Instruction) -> Option<Op> {
        match (instr.opcode(), instr.funct3()?) {
            (0x63, 0x00)        => Some(BranchEqual),
            (0x63, 0x01)        => Some(BranchNotEqual),
            (0x63, 0x04)        => Some(BranchLessThan),
            (0x63, 0x05)        => Some(BranchGreaterThanOrEqualTo),
            (0x63, 0x06)        => Some(BranchLessThanUnsigned),
            (0x63, 0x07)        => Some(BranchGreaterThanOrEqualToUnsigned),
            _                   => None,
        }
    }

    /// Decodes an I-type instruction.
    #[inline]
    fn decode_instr_i(instr: &Instruction) -> Option<Op> {
        match (instr.opcode(), instr.funct3()?, (instr.imm()? >> 0x05 & 0x7f)) {
            (0x03, 0x00, _)     => Some(LoadByte),
            (0x03, 0x01, _)     => Some(LoadHalf),
            (0x03, 0x02, _)     => Some(LoadWord),
            (0x03, 0x04, _)     => Some(LoadByteUnsigned),
            (0x03, 0x05, _)     => Some(LoadHalfUnsigned),
            (0x0f, 0x00, _)     => Some(Fence),
            (0x0f, 0x01, _)     => Some(FenceI),
            (0x13, 0x00, _)     => Some(ArithmeticAddImmediate),
            (0x13, 0x01, _)     => Some(ShiftLeftLogicalImmediate),
            (0x13, 0x02, _)     => Some(SetLessThanImmediate),
            (0x13, 0x03, _)     => Some(SetLessThanImmediateUnsigned),
            (0x13, 0x04, _)     => Some(LogicalExclusiveOrImmediate),
            (0x13, 0x05, 0x00)  => Some(ShiftRightLogicalImmediate),
            (0x13, 0x05, 0x20)  => Some(ShiftRightArithmeticImmediate),
            (0x13, 0x06, _)     => Some(LogicalOrImmediate),
            (0x13, 0x07, _)     => Some(LogicalAndImmediate),
            (0x67, 0x00, _)     => Some(JumpAndLinkRegister),
            // (0x73, 0x00, 0x00)  => Some(SystemEcall),
            // (0x73, 0x00, 0x01)  => Some(SystemEbreak),
            // (0x73, 0x01, _)     => Some(CsrReadWrite),
            // (0x73, 0x02, _)     => Some(CsrReadSet),
            // (0x73, 0x03, _)     => Some(CsrReadClear),
            // (0x73, 0x05, _)     => Some(CsrReadWriteImmediate),
            // (0x73, 0x06, _)     => Some(CsrReadSetImmediate),
            // (0x73, 0x07, _)     => Some(CsrReadClearImmediate),
            _                   => None,
        }
    }

    /// Decodes a J-type instruction.
    #[inline]
    fn decode_instr_j(instr: &Instruction) -> Option<Op> {
        match instr.opcode() {
            0x6f                => Some(JumpAndLink),
            _                   => None,
        }
    }

    /// Decodes an R-type instruction.
    #[inline]
    fn decode_instr_r(instr: & Instruction) -> Option<Op> {
        match (instr.opcode(), instr.funct3()?, instr.funct7()?) {
            (0x33, 0x00, 0x00)  => Some(ArithmeticAdd),
            (0x33, 0x00, 0x20)  => Some(ArithmeticSub),
            (0x33, 0x01, _)     => Some(ShiftLeftLogical),
            (0x33, 0x02, _)     => Some(SetLessThan),
            (0x33, 0x03, _)     => Some(SetLessThanUnsigned),
            (0x33, 0x04, _)     => Some(LogicalExclusiveOr),
            (0x33, 0x05, 0x00)  => Some(ShiftRightLogical),
            (0x33, 0x05, 0x20)  => Some(ShiftRightArithmetic),
            (0x33, 0x06, _)     => Some(LogicalOr),
            (0x33, 0x07, _)     => Some(LogicalAnd),
            _                   => None,
        }
    }

    /// Decodes an S-type instruction.
    #[inline]
    fn decode_instr_s(instr: &Instruction) -> Option<Op> {
        match (instr.opcode(), instr.funct3()?) {
            (0x23, 0x00)        => Some(StoreByte),
            (0x23, 0x01)        => Some(StoreHalf),
            (0x23, 0x02)        => Some(StoreWord),
            _                   => None,
        }
    }

    /// Decodes a U-type instruction.
    #[inline]
    fn decode_instr_u(instr: &Instruction) -> Option<Op> {
        match instr.opcode() {
            0x37                => Some(LoadUpperImmediate),
            _                   => None,
        }
    }

}