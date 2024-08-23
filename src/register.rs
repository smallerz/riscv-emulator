/// A generic implementation of a register.
/// The aim is to use this for both the general-purpose registers
/// and the floating-point registers.
#[derive(Clone, Copy, Debug)]
pub struct Register<T: Clone + Copy> {
    value: T
}

impl<T: Clone + Copy> Register<T> {
    /// Read the current value from the register.
    pub fn read(&self) -> T {
        self.value
    }

    /// Write a value to the register.
    pub fn write(&mut self, value: T) -> () {
        self.value = value;
    }
}

impl Register<u32> {
    pub fn new(initial_value: u32) -> Self {
        Self {
            value: initial_value
        }
    }
}

impl Default for Register<u32> {
    fn default() -> Self {
        Self {
            value: 0x00_u32
        }
    }
}

impl Default for Register<f32> {
    fn default() -> Self {
        Self {
            value: 0.0_f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn u32_register_value_defaults_to_zero() {
        let reg: Register<u32> = Register::default();
        assert_eq!(reg.read(), 0x00_u32);
    }

    #[test]
    fn f32_register_value_defaults_to_zero() {
        let reg: Register<f32> = Register::default();
        assert_eq!(reg.read(), 0.0_f32);
    }
}
