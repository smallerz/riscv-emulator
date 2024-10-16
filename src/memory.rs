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

    /// Reads one or more contiguous bytes from memory, starting from a base
    /// address. Addresses wrap around if the length exceeds the address space.
    pub fn read(&self, base_addr: usize, len: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);

        for i in 0 .. len {
            let index = self.wrap_addr(base_addr + i);
            result.push(self.data[index]);
        }

        result
    }

    /// Writes one or more bytes to memory contiguously, starting from a base
    /// address. Addresses wrap around if the length exceeds the address space.
    pub fn write(&mut self, base_addr: usize, value: &[u8]) {
        for i in 0 .. value.len() {
            let index = self.wrap_addr(base_addr + i);
            self.data[index] = value[i];
        }
    }

    /// Returns the size of the memory in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Wraps an address if it exceeds the address space.
    fn wrap_addr(&self, addr: usize) -> usize {
        addr % self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn memory_is_requested_length() {
        const SIZE: usize = 256;
        let mem = Memory::new(SIZE);
        assert_eq!(mem.len(), SIZE);
    }

    #[test]
    fn reads_the_requested_byte_count() {
        const SIZE: usize = 32;
        let mem = Memory::new(256);
        assert_eq!(mem.read(0, SIZE).len(), SIZE);
    }

    #[test]
    fn writes_the_requested_byte_count() {
        let mut mem = Memory::new(16);
        let data = &[1, 2, 3, 4];
        mem.write(7, data);
        assert_eq!(mem.read(7, 4), data);
    }
}