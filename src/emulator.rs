use crate::memory::Memory;
use crate::processor::Processor;

#[derive(Debug)]
pub struct EmulatorConfig {
    /// The size of main memory in bytes.
    pub mem_size: usize,

    /// The number of processors.
    pub proc_count: usize,
}

#[derive(Debug)]
pub struct Emulator {
    pub memory: Memory,
    pub proc: Vec<Processor>,
}

impl Emulator {
    pub fn build(config: EmulatorConfig) -> Self {
        Self {
            memory: Memory::new(config.mem_size),
            proc: (0 .. config.proc_count)
                .map(|_i| Processor::new())
                .collect(),
        }
    }
}
