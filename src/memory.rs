#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Creates a new instance of memory.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0x00; size],
        }
    }

    /// Read data from memory.
    /// Addresses wrap around if the length goes beyond the address space.
    pub fn read(&self, base_addr: usize, len: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);

        for i in 0 .. len {
            let index = (base_addr + i) % self.data.len();
            result.push(self.data[index]);
        }

        result
    }
}
