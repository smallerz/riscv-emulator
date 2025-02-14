use crate::op::{ Op, Op::* };

/// Arithmetic Logic Unit (ALU)
/// Responsible for performing arithmetic, logical and shift operations.
#[derive(Debug)]
pub struct Alu;

impl Alu {
    /// Creates a new ALU.
    pub fn new() -> Self {
        Alu {}
    }

    /// Performs an ALU operation on operands `x` and `y`.
    pub fn run(&self, op: &Op, x: i32, y: i32) -> i32 {
        match op {
            ArithmeticAdd | ArithmeticAddImmediate => {
                x.wrapping_add(y)
            },

            ArithmeticSub => {
                x.wrapping_sub(y)
            },

            LogicalAnd | LogicalAndImmediate => {
                x & y
            },

            LogicalOr | LogicalOrImmediate => {
                x | y
            },

            LogicalExclusiveOr | LogicalExclusiveOrImmediate => {
                x ^ y
            },

            ShiftLeftLogical | ShiftLeftLogicalImmediate => {
                x.wrapping_shl(y as u32)
            },

            ShiftRightArithmetic | ShiftRightArithmeticImmediate => {
                x.wrapping_shr(y as u32)
            },

            ShiftRightLogical | ShiftRightLogicalImmediate => {
                (x as u32).wrapping_shr(y as u32) as i32
            }

            _ => todo!(),
        }
    }
}

impl Default for Alu {
    fn default() -> Self {
        Alu::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{ Alu, Op::* };

    mod add {
        use super::*;

        #[test]
        fn adds_two_positive_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, 50, 50),
                100,
            );
        }
    
        #[test]
        fn adds_two_negative_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, -50, -50),
                -100,
            );
        }
    
        #[test]
        fn adds_positive_and_negative_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, 50, -50),
                0,
            );
        }
    
        #[test]
        fn adds_negative_and_positive_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, -50, 50),
                0,
            );
        }
    
        #[test]
        fn integer_overflow_wraps_around() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, i32::MAX, 1),
                i32::MIN,
            );
        }
    
        #[test]
        fn integer_underflow_wraps_around() {
            assert_eq!(
                Alu::default().run(&ArithmeticAdd, i32::MIN, -1),
                i32::MAX,
            );
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn subtracts_two_positive_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, 50, 50),
                0,
            );
        }
    
        #[test]
        fn subtracts_two_negative_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, -50, -50),
                0,
            );
        }
    
        #[test]
        fn subtracts_positive_and_negative_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, 50, -50),
                100,
            );
        }
    
        #[test]
        fn subtracts_negative_and_positive_integers() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, -50, 50),
                -100,
            );
        }
        
        #[test]
        fn integer_overflow_wraps_around() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, i32::MAX, -1),
                i32::MIN,
            );
        }
    
        #[test]
        fn integer_underflow_wraps_around() {
            assert_eq!(
                Alu::default().run(&ArithmeticSub, i32::MIN, 1),
                i32::MAX,
            );
        }
    }

    mod and {
        use super::*;

        #[test]
        fn zero_and_zero_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalAnd, 0b0, 0b0),
                0b0,
            );
        }

        #[test]
        fn zero_and_one_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalAnd, 0b0, 0b1),
                0b0,
            );
        }

        #[test]
        fn one_and_zero_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalAnd, 0b1, 0b0),
                0b0,
            );
        }

        #[test]
        fn one_and_one_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalAnd, 0b1, 0b1),
                0b1,
            );
        }
    }

    mod or {
        use super::*;

        #[test]
        fn zero_or_zero_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalOr, 0b0, 0b0),
                0b0,
            );
        }

        #[test]
        fn zero_or_one_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalOr, 0b0, 0b1),
                0b1,
            );
        }

        #[test]
        fn one_or_zero_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalOr, 0b1, 0b0),
                0b1,
            );
        }

        #[test]
        fn one_or_one_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalOr, 0b1, 0b1),
                0b1,
            );
        }
    }

    mod xor {
        use super::*;

        #[test]
        fn zero_xor_zero_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalExclusiveOr, 0b0, 0b0),
                0b0,
            );
        }

        #[test]
        fn zero_xor_one_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalExclusiveOr, 0b0, 0b1),
                0b1,
            );
        }

        #[test]
        fn one_xor_zero_equals_one() {
            assert_eq!(
                Alu::default().run(&LogicalExclusiveOr, 0b1, 0b0),
                0b1,
            );
        }

        #[test]
        fn one_xor_one_equals_zero() {
            assert_eq!(
                Alu::default().run(&LogicalExclusiveOr, 0b1, 0b1),
                0b0,
            );
        }
    }

    mod sll {
        use super::*;

        #[test]
        fn shifts_left() {
            assert_eq!(
                Alu::default().run(
                    &ShiftLeftLogical, 
                    0b10101010101010101010101010101010_u32 as i32, 
                    1,
                ),
                0b01010101010101010101010101010100,
            );
        }

        #[test]
        fn masks_shift_amount() {
            const X: u32 = 0xffffffff;
            let alu = Alu::default();

            assert_eq!(
                alu.run(
                    &ShiftLeftLogical,
                    X as i32,
                    33,
                ),
                alu.run(
                    &ShiftLeftLogical,
                    X as i32,
                    1,
                ),
            );
        }
    }

    mod sra {
        use super::*;

        #[test]
        fn shifts_right() {
            assert_eq!(
                Alu::default().run(
                    &ShiftRightArithmetic, 
                    0b10100000 as i32, 
                    4,
                ),
                0b00001010,
            );
        }

        #[test]
        fn sign_extends_positive() {
            assert_eq!(
                Alu::default().run(
                    &ShiftRightArithmetic, 
                    0b01111111111111111111111111111111 as i32, 
                    1,
                ),
                0b00111111111111111111111111111111,
            );
        }

        #[test]
        fn sign_extends_negative() {
            assert_eq!(
                Alu::default().run(
                    &ShiftRightArithmetic,
                    0b10000000000000000000000000000000_u32 as i32, 
                    1,
                ),
                0b11000000000000000000000000000000_u32 as i32,
            );
        }

        #[test]
        fn masks_shift_amount() {
            const X: u32 = 0xf0000000;
            let alu = Alu::default();

            assert_eq!(
                alu.run(
                    &ShiftRightArithmetic,
                    X as i32,
                    33,
                ),
                alu.run(
                    &ShiftRightArithmetic,
                    X as i32,
                    1,
                ),
            );
        }
    }

    mod srl {
        use super::*;

        #[test]
        fn shifts_right() {
            assert_eq!(
                Alu::default().run(
                    &ShiftRightLogical, 
                    0b10100000 as i32, 
                    4,
                ),
                0b00001010,
            );
        }

        #[test]
        fn zero_extends() {
            assert_eq!(
                Alu::default().run(
                    &ShiftRightLogical, 
                    0b11111111111111111111111111111111_u32 as i32, 
                    1,
                ),
                0b01111111111111111111111111111111_u32 as i32,
            );
        }

        #[test]
        fn masks_shift_amount() {
            const X: u32 = 0xf0000000;
            let alu = Alu::default();

            assert_eq!(
                alu.run(
                    &ShiftRightLogical,
                    X as i32,
                    33,
                ),
                alu.run(
                    &ShiftRightLogical,
                    X as i32,
                    1,
                ),
            );
        }
    }
}