use crate::memory::Memory;
use crate::processor::Processor;

/// A RISC-V emulator.
#[derive(Debug)]
pub struct Emulator {
    config: EmulatorConfig,
}

/// Settings that determine the emulator's behaviour.
#[derive(Debug)]
pub struct EmulatorConfig {
    /// Determines whether debug mode is enabled.
    pub is_debug_enabled: bool,
    /// The size of the emulator's memory in bytes.
    pub memory_size: usize,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        EmulatorConfig {
            is_debug_enabled: false,
            memory_size: 1024,
        }
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            config: EmulatorConfig::default(),
        }
    }
}

impl Emulator {
    /// Creates a new RISC-V emulator.
    pub fn new(config: EmulatorConfig) -> Self {
        Emulator {
            config,
        }
    }

    /// Starts the emulator.
    pub fn start(&self) {
        let mem = Memory::new(self.config.memory_size);
        let mut proc = Processor::new();

        loop {
            let instr = proc.fetch();
            // proc.execute() handles instruction decoding.
            proc.execute(&instr);
        }
    }
}
