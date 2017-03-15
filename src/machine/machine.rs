use std::fs::File;
use machine::cpu::Cpu;

pub struct Machine {
    cpu: Cpu,
}

impl Machine {
    pub fn new() -> Machine {
        let machine = Machine {
            cpu: Cpu::new(),
        };
        machine
    }

    pub fn load_bin(&mut self, bin: File) {
        self.cpu.load_bin(bin);
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle();
    }
}
