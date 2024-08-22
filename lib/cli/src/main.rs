use riscv_emulator::add;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use riscv_emulator::add;

    #[test]
    fn it_also_works() -> () {
        assert_eq!(add(1, 1), 2);
    }
}