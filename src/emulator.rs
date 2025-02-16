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
    pub fn dev_start(&mut self, obj_data: &[u8]) {
        println!("Instructions:\n");

        obj_data
            .chunks_exact(4)
            .for_each(|word: &[u8]| {
                let instr = Instruction::new(
                    u32::from_le_bytes(
                        [word[0], word[1], word[2], word[3]]
                    )
                );

                println!("{instr}");

                self.proc[0].execute(&instr);
            });

        println!(
            "\nProgram Counter:\t{:#010x} / {}", 
            self.proc[0].pc,
            self.proc[0].pc, 
        );

        println!("\nRegisters:\n");

        (0 .. self.proc[0].reg_x.len() / 2)
            .for_each(|i| {
                println!(
                    "x{:<2}  ( 0x{:08x} )    x{:<2}  ( 0x{:08x} )",
                    i,
                    self.proc[0].reg_x.read(i),
                    i + 16,
                    self.proc[0].reg_x.read(i + 16),
                );
            });
    }
}
