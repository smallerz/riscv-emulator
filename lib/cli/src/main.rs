use std::{fs::File, io::{Error, Read}, process};

use clap::Parser;

use riscv_emulator::emulator::{
    Emulator,
    EmulatorConfig,
};

#[derive(Debug, Parser)]
struct Args {
    /// The RISC-V ELF binary to execute
    #[arg()]
    input_file: String,

    /// The size of the emulator's memory in bytes
    #[arg(short, long, default_value_t = 1024)]
    memory_size: usize,
}

fn main() {
    let args = Args::parse();

    let config = EmulatorConfig { 
        mem_size: args.memory_size,
        proc_count: 1,
    };

    let mut emu = Emulator::build(config);

    let data = dev_read_input_file(&args.input_file)
        .unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

    emu.dev_start(&data);
}

fn dev_read_input_file(path: &str) -> Result<Vec<u8>, Error> {
    let file = File::open(path);
    let mut buf = Vec::new();
    file?.read_to_end(&mut buf)?;
    Ok(buf)
}
