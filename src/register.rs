/// An alias for the RISC-V general purpose registers.
pub type RegistersX = Registers<u32, 32>;

/// The read/write access level of a register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessLevel {
    Read,
    ReadWrite,
}

/// A group of registers of a generic size and quantity.
#[derive(Debug)]
pub struct Registers<T, const U: usize> {
    access_levels: [AccessLevel; U],
    values: [T; U],    
}

impl<T: Copy + Default, const U: usize> Registers<T, U> {
    /// Create a new group of registers of type `T` and of quantity `U`.
    pub fn new() -> Registers<T, U> {
        Self {
            access_levels: [AccessLevel::Read; U],
            values: [T::default(); U],
        }
    }

    /// Returns the current value of a register at a given index.
    pub fn read(&self, index: usize) -> T {
        self.values[index]
    }

    /// Writes a value to a register at a given index.
    pub fn write(&mut self, index: usize, value: T) {
        if !self.is_read_only(index) {
            self.values[index] = value;
        }
    }

    /// Resets all registers to their default values.
    pub fn reset(&mut self) {
        self.values.fill(T::default());
    }

    /// Returns the number of registers in the collection.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the register at a given index is read-only.
    pub fn is_read_only(&self, index: usize) -> bool {
        self.access_levels[index] == AccessLevel::Read
    }

    /// Sets the access level of a register at a given index.
    pub fn set_access_level(&mut self, index: usize, access_level: AccessLevel) {
        self.access_levels[index] = access_level;
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessLevel, Registers};

    const REG_COUNT: usize = 32;

    #[test]
    #[should_panic]
    fn panics_on_attempt_to_read_from_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new().read(REG_COUNT);
    }

    #[test]
    #[should_panic]
    fn panics_on_attempt_to_write_to_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new().write(REG_COUNT * 2, 0x00);
    }

    #[test]
    #[should_panic]
    fn panics_on_attempt_to_set_access_level_of_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new()
            .set_access_level(REG_COUNT * 2, AccessLevel::Read);
    }

    #[test]
    #[should_panic]
    fn panics_on_attempt_to_get_read_only_access_of_nonexistent_register() {
        Registers::<u32, REG_COUNT>::new()
            .is_read_only(REG_COUNT * 2);
    }

    #[test]
    fn default_register_access_level_is_read_only() {
        assert_eq!(
            Registers::<u32, REG_COUNT>::new().is_read_only(0), 
            true
        );
    }

    #[test]
    fn ignores_write_attempt_to_register_with_read_only_access() {
        let mut regs = Registers::<u32, REG_COUNT>::new();
        regs.write(0, 0xff);
        assert_eq!(regs.read(0), 0x00);
    }

    #[test]
    fn writes_to_register_with_read_write_access() {
        const INDEX: usize = 9;
        const VALUE: u32 = 0xff;

        let mut regs = Registers::<u32, REG_COUNT>::new();
        regs.set_access_level(INDEX, AccessLevel::ReadWrite);
        regs.write(INDEX, VALUE);
        assert_eq!(regs.read(INDEX), VALUE);
    }

    #[test]
    fn resets_register_values() {
        const INDEX: usize = 15;
        const VALUE: u32 = 0xff;

        let mut regs = Registers::<u32, REG_COUNT>::new();
        regs.set_access_level(INDEX, AccessLevel::ReadWrite);
        regs.write(INDEX, VALUE);
        assert_eq!(regs.read(INDEX), VALUE);
        regs.reset();
        assert_eq!(regs.read(INDEX), u32::default());
    }

    #[test]
    fn len_returns_correct_register_count() {
        assert_eq!(
            Registers::<u32, REG_COUNT>::new().len(),
            REG_COUNT
        )
    }
}
