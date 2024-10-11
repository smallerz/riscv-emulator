/// Arithmetic Logic Unit (ALU)
/// Responsible for performing arithmetic operations.
#[derive(Debug)]
pub struct ALU;

impl ALU {
    /// Adds two unsigned integers and returns the result.
    pub fn add(&self, x: i32, y: i32) -> i32 {
        x.wrapping_add(y)
    }
}
