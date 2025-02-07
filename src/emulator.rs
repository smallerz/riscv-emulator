use crate::instruction::Instruction;
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

    // Just for testing purposes. Will delete later.
    pub fn debug(&mut self) {
        let prog: Vec<u32> = vec![
            // x10 = x0 + 45
            0x02d00513, 
            // x10 = x10 + 10
            0x00a50513,
            // x10 = x10 + -5
            0xffb50513
        ];

        prog.iter()
            .for_each(|i| {
                self.proc[0].execute(&Instruction::new(*i))
            });

        (0 .. self.proc[0].reg_x.len())
            .for_each(|i| {
                println!("x{i}\t{}", self.proc[0].reg_x.read(i));
            });
    }
}
