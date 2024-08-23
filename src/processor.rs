use crate::register::Register;

const REGISTER_COUNT: usize = 32;

/// The main CPU instance.
#[derive(Debug)]
pub struct Processor {
    /// Program Counter
    //pc: u64,

    /// Integer Registers
    pub registers_x: [Register<u32>; REGISTER_COUNT],

    /// Floating-Point Registers
    pub registers_f: [Register<f32>; REGISTER_COUNT],
}

impl Processor {
    pub fn new() -> Self {
        Self {
            registers_x: [Register::default(); REGISTER_COUNT],
            registers_f: [Register::default(); REGISTER_COUNT],
        }
    }
}
