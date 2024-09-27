/// Arithmetic Logic Unit (ALU)
/// Responsible for performing arithmetic operations.
#[derive(Debug)]
pub struct ALU;

impl ALU {
    /// `add` instruction.
    /// Adds the values in two source registers (`rs1` + `rs2`) and 
    /// stores the result in a destination register (`rd`).
    pub fn add(&self, x: u32, y: u32) -> u32 {
        x.wrapping_add(y)
    }
}
