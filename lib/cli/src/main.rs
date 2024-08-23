use riscv_emulator::processor::Processor;

fn main() {
    let proc = Processor::new();
    println!("{:?}", proc.registers_x[0]);
}

#[cfg(test)]
mod tests {

}