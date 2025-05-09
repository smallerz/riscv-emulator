use std::fmt::Display;

use Op::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    AddUpperImmediateProgramCounter,
    ArithmeticAdd,
    ArithmeticAddImmediate,
    ArithmeticSub,
    BranchEqual,
    BranchGreaterThanOrEqualTo,
    BranchGreaterThanOrEqualToUnsigned,
    BranchLessThan,
    BranchLessThanUnsigned,
    BranchNotEqual,
    // CsrReadClear,
    // CsrReadClearImmediate,
    // CsrReadSet,
    // CsrReadSetImmediate,
    // CsrReadWrite,
    // CsrReadWriteImmediate,
    Fence,
    FenceI,
    JumpAndLink,
    JumpAndLinkRegister,
    LoadByte,
    LoadByteUnsigned,
    LoadHalf,
    LoadHalfUnsigned,
    LoadUpperImmediate,
    LoadWord,
    LogicalAnd,
    LogicalAndImmediate,
    LogicalExclusiveOr,
    LogicalExclusiveOrImmediate,
    LogicalOr,
    LogicalOrImmediate,
    SetLessThan,
    SetLessThanImmediate,
    SetLessThanImmediateUnsigned,
    SetLessThanUnsigned,
    ShiftLeftLogical,
    ShiftLeftLogicalImmediate,
    ShiftRightArithmetic,
    ShiftRightArithmeticImmediate,
    ShiftRightLogical,
    ShiftRightLogicalImmediate,
    StoreByte,
    StoreHalf,
    StoreWord,
    // SystemEbreak,
    // SystemEcall,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddUpperImmediateProgramCounter     => "auipc",
                ArithmeticAdd                       => "add",
                ArithmeticAddImmediate              => "addi",
                ArithmeticSub                       => "sub",
                BranchEqual                         => "beq",
                BranchGreaterThanOrEqualTo          => "bge",
                BranchGreaterThanOrEqualToUnsigned  => "bgeu",
                BranchLessThan                      => "blt",
                BranchLessThanUnsigned              => "bltu",
                BranchNotEqual                      => "bne",
                // CsrReadClear                        => "csrrc",
                // CsrReadClearImmediate               => "csrrci",
                // CsrReadSet                          => "csrrs",
                // CsrReadSetImmediate                 => "csrrsi",
                // CsrReadWrite                        => "csrw",
                // CsrReadWriteImmediate               => "csrwi",
                Fence                               => "fence",
                FenceI                              => "fence.i",
                JumpAndLink                         => "jal",
                JumpAndLinkRegister                 => "jalr",
                LoadByte                            => "lb",
                LoadByteUnsigned                    => "lbu",
                LoadHalf                            => "lh",
                LoadHalfUnsigned                    => "lhu",
                LoadUpperImmediate                  => "lui",
                LoadWord                            => "lw",
                LogicalAnd                          => "and",
                LogicalAndImmediate                 => "andi",
                LogicalExclusiveOr                  => "xor",
                LogicalExclusiveOrImmediate         => "xori",
                LogicalOr                           => "or",
                LogicalOrImmediate                  => "ori",
                SetLessThan                         => "slt",
                SetLessThanImmediate                => "slti",
                SetLessThanImmediateUnsigned        => "sltiu",
                SetLessThanUnsigned                 => "sltu",
                ShiftLeftLogical                    => "sll",
                ShiftLeftLogicalImmediate           => "slli",
                ShiftRightArithmetic                => "sra",
                ShiftRightArithmeticImmediate       => "srai",
                ShiftRightLogical                   => "srl",
                ShiftRightLogicalImmediate          => "srli",
                StoreByte                           => "sb",
                StoreHalf                           => "sh",
                StoreWord                           => "sw",
                // SystemEbreak                        => "ebreak",
                // SystemEcall                         => "ecall",
            }
        )
    }
}