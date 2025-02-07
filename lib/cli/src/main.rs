use riscv_emulator::emulator::{
    Emulator,
    EmulatorConfig,
};

fn main() {
    let config = EmulatorConfig { 
        mem_size: 1024,
        proc_count: 1,
    };

    let mut emu = Emulator::build(config);

    println!("Memory (B):\t{}", emu.memory.len());
    println!("Processors:\t{}", emu.proc.len());

    emu.debug();
}
