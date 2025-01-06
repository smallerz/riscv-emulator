pub type RegistersX = Registers<u32, 32>;

/// A group of registers of a generic size.
#[derive(Debug)]
pub struct Registers<T, const U: usize> {
    read_only_registers: Vec<usize>,
    values: [T; U],    
}

impl<T: Copy + Default, const U: usize> Registers<T, U> {
    /// Create a new group of U T-bit registers.
    pub fn new(read_only_registers: &[usize]) -> Registers<T, U> {
        Self {
            read_only_registers: read_only_registers.to_vec(),
            values: [T::default(); U],
        }
    }
}

impl<T: Copy, const U: usize> Registers<T, U> {
    /// Reads a value from a register.
    pub fn read(&self, index: usize) -> T {
        self.values[index]
    }

    /// Writes a value to a register.
    pub fn write(&mut self, index: usize, value: T) {
        if !self.read_only_registers.contains(&index) {
            self.values[index] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    const REG_COUNT: usize = 32;

    #[test]
    #[should_panic]
    fn panics_if_attempting_to_read_from_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new(&[]).read(REG_COUNT);
    }

    #[test]
    #[should_panic]
    fn panics_if_attempting_to_write_to_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new(&[]).write(REG_COUNT, 0x00);
    }

    #[test]
    fn zero_register_is_zero() {
        assert_eq!(
            Registers::<u32, REG_COUNT>::new(&[0]).read(0),
            0x00
        )
    }

    #[test]
    fn ignores_write_attempts_to_zero_register() {
        let mut regs = Registers::<u32, REG_COUNT>::new(&[0]);
        regs.write(0, 0xff);
        assert_eq!(regs.read(0), 0x00);
    }

    #[test]
    fn writes_to_nonzero_register() {
        let mut regs = Registers::<u32, REG_COUNT>::new(&[]);
        regs.write(9, 0xff);
        assert_eq!(regs.read(9), 0xff);
    }
}