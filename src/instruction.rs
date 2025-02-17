use std::fmt::Display;

use crate::decode::Decoder;

use InstructionFormat::*;

/// RISC-V's instruction formats, which indicate how instructions
/// are encoded and the fields that they contain.
#[derive(Debug, Eq, PartialEq)]
pub enum InstructionFormat {
    /// ## B-type instruction format (Branch)
    /// `opcode`, `imm[11|1:4]`, `funct3`, `rs1`, `rs2`, `imm[5:10|12]`
    B,
    
    /// ## I-type instruction format (Immediate)
    /// `opcode`, `rd`, `funct3`, `rs1`, `imm`
    I,
    
    /// ## J-type instruction format (Jump)
    /// `opcode`, `rd`, `imm[12:19|11|1:10|20]`
    J,

    /// ## R-type instruction format (Register)
    /// `opcode`, `rd`, `funct3`, `rs1`, `rs2`, `funct7`
    R,
    
    /// ## S-type instruction format (Store)
    /// `opcode`, `imm[0:4]`, `funct3`, `rs1`, `rs2`, `imm[5:11]`
    S,

    /// ## U-type instruction format (Upper-Immediate)
    /// `opcode`, `rd`, `imm[20]`
    U,
}

/// A 32-bit RISC-V instruction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instruction {
    /// The 32-bit instruction value.
    instr: u32
}

impl Instruction {
    /// Creates a new instruction from an unsigned 32-bit integer.
    pub fn new(instr: u32) -> Self {
        Instruction { instr }
    }

    /// Returns the format of the instruction.
    pub fn format(&self) -> InstructionFormat {
        match self.opcode() {
            0x03 | 0x0f | 0x13 | 0x17 | 0x67 | 0x73 => I,
            0x23 => S,
            0x33 => R,
            0x37 => U,
            0x63 => B,
            0x6f => J,
            _ => todo!(
                "Invalid instruction format handler not yet implemented"),
        }
    }

    /// Returns the mnemonic associated with the instruction.
    pub fn mnemonic(&self) -> String {
        Decoder::decode(self).unwrap().to_string()
    }

    /// Returns the instruction's opcode field.
    pub fn opcode(&self) -> u8 {
        (self.instr & 0x7f) as u8
    }

    /// Returns the value of the instruction's rd field,
    /// or None if the instruction doesn't have an rd field.
    pub fn rd(&self) -> Option<usize> {
        match self.format() {
            I | J | R | U => Some((self.instr >> 7 & 0x1f) as usize),
            _ => None,
        }
    }

    /// Returns the value of the instruction's funct3 field,
    /// or None if the instruction doesn't have an funct3 field.
    pub fn funct3(&self) -> Option<u8> {
        match self.format() {
            B | I | R | S => Some((self.instr >> 12 & 0x07) as u8),
            _ => None,
        }
    }

    /// Returns the value of the instruction's funct7 field,
    /// or None if the instruction doesn't have an funct7 field.
    pub fn funct7(&self) -> Option<u8> {
        match self.format() {
            R => Some((self.instr >> 25 & 0x7f) as u8),
            _ => None,
        }
    }

    /// Returns the value of the instruction's rs1 field,
    /// or None if the instruction doesn't have an rs1 field.
    pub fn rs1(&self) -> Option<usize> {
        match self.format() {
            B | I | R | S => Some((self.instr >> 15 & 0x1f) as usize),
            _ => None,
        }
    }

    /// Returns the value of the instruction's rs2 field,
    /// or None if the instruction doesn't have an rs2 field.
    pub fn rs2(&self) -> Option<usize> {
        match self.format() {
            B | R | S => Some((self.instr >> 20 & 0x1f) as usize),
            _ => None,
        }
    }

    /// Returns the value of the instruction's imm field,
    /// or None if the instruction doesn't have an imm field.
    pub fn imm(&self) -> Option<i32> {
        match self.format() {
            B => Some(self.imm_b()),
            I => Some(self.imm_i()),
            J => Some(self.imm_j()),
            S => Some(self.imm_s()),
            U => Some(self.imm_u()),
            _ => None,
        }
    }

    #[inline]
    fn imm_b(&self) -> i32 {
        Instruction::sign_ext(
            // imm[1:4]
            (self.instr >> 8 & 0x0f) << 1
                // imm[5:10]
                | (self.instr >> 25 & 0x3f) << 5
                // imm[11]
                | (self.instr >> 7 & 0x01) << 11
                // imm[12]
                | (self.instr >> 31 & 0x01) << 12,
            12
        )
    }

    #[inline]
    fn imm_i(&self) -> i32 {
        Instruction::sign_ext(
            self.instr >> 20 & 0xfff,
            12
        )
    }

    #[inline]
    fn imm_j(&self) -> i32 {
        Instruction::sign_ext(
            // imm[1:10]
            (self.instr >> 21 & 0x3ff) << 1
                // imm[11]
                | (self.instr >> 20 & 0x01) << 11
                // imm[12:19]
                | (self.instr >> 12 & 0xff) << 12
                // imm[20]
                | (self.instr >> 31 & 0x01) << 20,
            20
        )
    }

    #[inline]
    fn imm_s(&self) -> i32 {
        Instruction::sign_ext(
            // imm[0:4]
            (self.instr >> 7 & 0x1f)
                // imm[5:11]
                | (self.instr >> 25 & 0x7f) << 5,
            12
        )
    }

    #[inline]
    fn imm_u(&self) -> i32 {
        Instruction::sign_ext(
            self.instr >> 12 & 0xfffff,
            20
        )
    }

    /// Sign-extend an instruction field.
    fn sign_ext(value: u32, field_size: usize) -> i32 {
        ((value << (32 - field_size)) as i32) >> (32 - field_size)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.format() {
                B => format!(
                    // mnemonic rs1, rs2, imm
                    "{:<12} x{}, x{}, {:#010x}",
                    self.mnemonic(),
                    self.rs1().unwrap(),
                    self.rs2().unwrap(),
                    self.imm().unwrap(),
                ),

                I => {
                    match self.opcode() {
                        0x03 | 0x67 => {
                            format!(
                                // mnemonic rd, imm(rs1)
                                "{:<12} x{:}, {}(x{})",
                                self.mnemonic(),
                                self.rd().unwrap(),
                                self.imm().unwrap(),
                                self.rs1().unwrap(),
                            )
                        }
                        _ => {
                            format!(
                                // mnemonic rd, rs1, imm
                                "{:<12} x{:}, x{}, {:#010x}",
                                self.mnemonic(),
                                self.rd().unwrap(),
                                self.rs1().unwrap(),
                                self.imm().unwrap(),
                            )
                        }
                    }
                },

                J => format!(
                    // mnemonic rd, imm
                    "{:<12} x{}, {:#010x}",
                    self.mnemonic(),
                    self.rd().unwrap(),
                    self.imm().unwrap(),
                ),
                
                R => format!(
                    // mnemonic rd, rs1, rs2
                    "{:<12} x{}, x{}, x{}",
                    self.mnemonic(),
                    self.rd().unwrap(),
                    self.rs1().unwrap(),
                    self.rs2().unwrap(),
                ),

                S => format!(
                    // mnemonic rs2, imm(rs1)
                    "{:<12} x{}, {:#010x}(x{})",
                    self.mnemonic(),
                    self.rs2().unwrap(),
                    self.imm().unwrap(),
                    self.rs1().unwrap(),
                ),
                
                U => format!(
                    // mnemonic rd, imm
                    "{:<12} x{}, {:#010x}",
                    self.mnemonic(),
                    self.rd().unwrap(),
                    self.imm().unwrap(),
                ),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Instruction,
        InstructionFormat::*,
    };

    mod b_type {
        use super::*;

        // bne      x9, x11, 20
        // opcode:  0x63,
        // funct3:  0x01,
        // rs1:     0x09,
        // rs2:     0x0b,
        // imm:     0x14,
        const B_INSTR: u32 = 0x00b49a63;

        #[test]
        fn has_b_format() {
            assert_eq!(Instruction::new(B_INSTR).format(), B);
        }
    
        #[test]
        fn has_valid_opcode() {
            assert_eq!(Instruction::new(B_INSTR).opcode(), 0x63);
        }
    
        #[test]
        fn has_no_rd() {
            assert_eq!(Instruction::new(B_INSTR).rd(), None);
        }
    
        #[test]
        fn has_funct3() {
            assert_eq!(Instruction::new(B_INSTR).funct3(), Some(0x01));
        }
    
        #[test]
        fn has_rs1() {
            assert_eq!(Instruction::new(B_INSTR).rs1(), Some(0x09));
        }
    
        #[test]
        fn has_rs2() {
            assert_eq!(Instruction::new(B_INSTR).rs2(), Some(0x0b));
        }
    
        #[test]
        fn has_no_funct7() {
            assert_eq!(Instruction::new(B_INSTR).funct7(), None);
        }
        
        #[test]
        fn has_imm() {
            assert_eq!(Instruction::new(B_INSTR).imm(), Some(0x14));
        }
    
        #[test]
        fn sign_extends_imm() {
            let imm = Instruction::new(0xfe529ae3).imm().unwrap();
            assert_eq!(imm.is_negative(), true);
        }
    }

    mod i_type {
        use super::*;

        // addi     x10, x11 -12
        // opcode:  0x13,
        // rd:      0x0a,
        // funct3:  0x00,
        // rs1:     0x0b,
        // imm:     0xff4,
        const I_INSTR: u32 = 0xff458513;
        
            #[test]
            fn has_i_format() {
                assert_eq!(Instruction::new(I_INSTR).format(), I);
            }
        
            #[test]
            fn has_valid_opcode() {
                assert_eq!(Instruction::new(I_INSTR).opcode(), 0x13);
            }
        
            #[test]
            fn has_rd() {
                assert_eq!(Instruction::new(I_INSTR).rd(), Some(0x0a));
            }
        
            #[test]
            fn has_funct3() {
                assert_eq!(Instruction::new(I_INSTR).funct3(), Some(0x00));
            }
        
            #[test]
            fn has_rs1() {
                assert_eq!(Instruction::new(I_INSTR).rs1(), Some(0x0b));
            }
        
            #[test]
            fn has_no_rs2() {
                assert_eq!(Instruction::new(I_INSTR).rs2(), None);
            }
        
            #[test]
            fn has_no_funct7() {
                assert_eq!(Instruction::new(I_INSTR).funct7(), None);
            }
            
            #[test]
            fn has_imm() {
                assert_eq!(Instruction::new(I_INSTR).imm(), Some(-12));
            }
        
            #[test]
            fn sign_extends_imm() {
                // addi x2, x2, -32
                let instr = 0xfe010113;
        
                let imm = Instruction::new(instr).imm().unwrap();
        
                assert_eq!(imm.is_negative(), true);
            }
    }

    mod j_type {
        use super::*;

            // jal      x0, 64
            // opcode:  0x6f,
            // rd:      0x00,
            // imm:     0x40,
            const J_INSTR: u32 = 0x0400006f;

            #[test]
            fn has_j_format() {
                assert_eq!(Instruction::new(J_INSTR).format(), J);
            }
        
            #[test]
            fn has_valid_opcode() {
                assert_eq!(Instruction::new(J_INSTR).opcode(), 0x6f);
            }
        
            #[test]
            fn has_rd() {
                assert_eq!(Instruction::new(J_INSTR).rd(), Some(0x00));
            }
        
            #[test]
            fn has_no_funct3() {
                assert_eq!(Instruction::new(J_INSTR).funct3(), None);
            }
        
            #[test]
            fn has_no_rs1() {
                assert_eq!(Instruction::new(J_INSTR).rs1(), None);
            }
        
            #[test]
            fn has_no_rs2() {
                assert_eq!(Instruction::new(J_INSTR).rs2(), None);
            }
        
            #[test]
            fn has_no_funct7() {
                assert_eq!(Instruction::new(J_INSTR).funct7(), None);
            }
            
            #[test]
            fn has_imm() {
                assert_eq!(Instruction::new(J_INSTR).imm(), Some(0x40));
            }
        
            #[test]
            fn sign_extends_imm() {
                // jal x0, -391854
                let instr = 0xd52a006f;
        
                let imm = Instruction::new(instr).imm().unwrap();
        
                assert_eq!(imm.is_negative(), true);
            }
    }

    mod r_type {
        use super::*;

            // sub      x5, x7, x3
            // opcode:  0x33, 
            // rd:      0x05, 
            // funct3:  0x00, 
            // rs1:     0x07, 
            // rs2:     0x03, 
            // funct7:  0x20,
            const R_INSTR: u32 = 0x403382b3;

            #[test]
            fn has_r_format() {
                assert_eq!(Instruction::new(R_INSTR).format(), R);
            }
        
            #[test]
            fn has_valid_opcode() {
                assert_eq!(Instruction::new(R_INSTR).opcode(), 0x33);
            }
        
            #[test]
            fn has_rd() {
                assert_eq!(Instruction::new(R_INSTR).rd(), Some(0x05));
            }
        
            #[test]
            fn has_funct3() {
                assert_eq!(Instruction::new(R_INSTR).funct3(), Some(0x00));
            }
        
            #[test]
            fn has_rs1() {
                assert_eq!(Instruction::new(R_INSTR).rs1(), Some(0x07));
            }
        
            #[test]
            fn has_rs2() {
                assert_eq!(Instruction::new(R_INSTR).rs2(), Some(0x03));
            }
        
            #[test]
            fn has_funct7() {
                assert_eq!(Instruction::new(R_INSTR).funct7(), Some(0x20));
            }
            
            #[test]
            fn has_no_imm() {
                assert_eq!(Instruction::new(R_INSTR).imm(), None);
            }
    }

    mod s_type {
        use super::*;

        // sw       x6, 4(x12)
        // opcode:  0x23,
        // funct3:  0x02,
        // rs1:     0x0c,
        // rs2:     0x06,
        // imm:     0x04,
        const S_INSTR: u32 = 0x00662223;

        #[test]
        fn has_s_format() {
            assert_eq!(Instruction::new(S_INSTR).format(), S);
        }
    
        #[test]
        fn has_valid_opcode() {
            assert_eq!(Instruction::new(S_INSTR).opcode(), 0x23);
        }
    
        #[test]
        fn has_no_rd() {
            assert_eq!(Instruction::new(S_INSTR).rd(), None);
        }
    
        #[test]
        fn has_funct3() {
            assert_eq!(Instruction::new(S_INSTR).funct3(), Some(0x02));
        }
    
        #[test]
        fn has_rs1() {
            assert_eq!(Instruction::new(S_INSTR).rs1(), Some(0x0c));
        }
    
        #[test]
        fn has_rs2() {
            assert_eq!(Instruction::new(S_INSTR).rs2(), Some(0x06));
        }
    
        #[test]
        fn has_no_funct7() {
            assert_eq!(Instruction::new(S_INSTR).funct7(), None);
        }
        
        #[test]
        fn has_imm() {
            assert_eq!(Instruction::new(S_INSTR).imm(), Some(0x04));
        }
    
        #[test]
        fn sign_extends_imm() {
            // sh x29, -28(x5)
            let instr = 0xffd29223;
    
            let imm = Instruction::new(instr).imm().unwrap();
            assert_eq!(imm.is_negative(), true);
        }
    }

    mod u_type {
        use super::*;

        // lui      x10, 0xfffff
        // opcode:  0x37,
        // rd:      0x0a,
        // imm:     0xfffff
        const U_INSTR: u32 = 0xfffff537;    
    
        #[test]
        fn has_u_format() {
            assert_eq!(Instruction::new(U_INSTR).format(), U);
        }
    
        #[test]
        fn has_valid_opcode() {
            assert_eq!(Instruction::new(U_INSTR).opcode(), 0x37);
        }
    
        #[test]
        fn has_rd() {
            assert_eq!(Instruction::new(U_INSTR).rd(), Some(0x0a));
        }
    
        #[test]
        fn has_no_funct3() {
            assert_eq!(Instruction::new(U_INSTR).funct3(), None);
        }
    
        #[test]
        fn has_no_rs1() {
            assert_eq!(Instruction::new(U_INSTR).rs1(), None);
        }
    
        #[test]
        fn has_no_rs2() {
            assert_eq!(Instruction::new(U_INSTR).rs2(), None);
        }
    
        #[test]
        fn has_no_funct7() {
            assert_eq!(Instruction::new(U_INSTR).funct7(), None);
        }
        
        #[test]
        fn has_imm() {
            assert_eq!(Instruction::new(U_INSTR).imm(), Some(-1));
        }
    
        #[test]
        fn sign_extends_imm() {
            let imm = Instruction::new(U_INSTR).imm().unwrap();
            assert_eq!(imm.is_negative(), true);
        }
    }
}
