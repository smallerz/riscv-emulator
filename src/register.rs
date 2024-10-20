/// A group of registers of a generic size.
pub struct Registers<T> {
    values: [T; 32],    
}

impl Registers<u32> {
    /// Create a new group of 32-bit registers.
    pub fn new() -> Registers<u32> {
        Self {
            values: [0x00_u32; 32],
        }
    }
}

// impl Registers<u64> {
//     pub fn new() -> Registers<u64> {
//         Self {
//             values: [0x00_u64; 32],
//         }
//     }
// }

impl<T: Copy> Registers<T> {
    /// Reads a value from a register.
    pub fn read(&self, index: usize) -> T {
        self.values[index]
    }

    /// Writes a value to a register.
    pub fn write(&mut self, index: usize, value: T) {
        if index != 0 {
            self.values[index] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    fn zero_register_is_zero() {
        assert_eq!(
            Registers::<u32>::new().read(0),
            0x00
        )
    }

    #[test]
    fn ignores_write_attempts_to_zero_register() {
        let mut regs = Registers::<u32>::new();
        regs.write(0, 0xff);
        assert_eq!(regs.read(0), 0x00);
    }

    #[test]
    fn writes_to_nonzero_register() {
        let mut regs = Registers::<u32>::new();
        regs.write(9, 0xff);
        assert_eq!(regs.read(9), 0xff);
    }
}