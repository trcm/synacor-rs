mod machine;
use machine::cpu::Cpu;
use std::fs::File;
use std::str::FromStr;

fn main() {
    println!("Hello, Synacor World!");
    let mut vm = Cpu::new();
    
    let bin = match File::open("./challenge.bin") {
        Ok(f) => f,
        Err(e) => panic!("Err {}", e),
    };

    vm.load_bin(bin);
    loop {
        vm.cycle();
    }
}
