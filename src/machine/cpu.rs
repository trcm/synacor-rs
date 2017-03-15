use std::io;
use std::io::prelude::*;
use std::process::exit;
use std::fs::File;
use machine::op::Opcode;

const MEM_SIZE: u16 = 32768;

pub struct Cpu {
    opcode: u16,
    pc: u16,
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    sp: u16,
    pub stack: Vec<u16>,
    memory: [u16; 32768]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0x00,
            pc: 0x00,
            r0: 0x00,
            r1: 0x00,
            r2: 0x00,
            r3: 0x00,
            r4: 0x00,
            r5: 0x00,
            r6: 0x00,
            r7: 0x00,
            sp: 0x00,
            stack: Vec::new(),
            memory: [0; 32768],
        }
    }

    pub fn load_bin(&mut self, bin: File) {
        let mut count = 0;
        let mut bytes = bin.bytes().peekable();
        
        loop {
            if bytes.peek().is_none() {
                break;
            }
            let lower = match bytes.next() {
                Some(ok) => ok.unwrap(),
                None => panic!(),
            };
            let upper = match bytes.next() {
                Some(ok) => ok.unwrap(),
                None => panic!(),
            };
            let value = (upper as u16) << 8 | lower as u16;
            self.memory[count] = value;
            count += 1;
        }
    }

    pub fn get_opcode(&mut self) -> u16 {
        self.memory[self.pc as usize]
    }

    fn read_location(&mut self, step: u16) -> u16 {
        let number = self.memory[(self.pc + step) as usize];
        self.parse_number(number)
    }

    pub fn get_reg(&mut self, register: u16) -> u16 {
        match register {
            32768 => self.r0,
            32769 => self.r1,
            32770 => self.r2,
            32771=> self.r3,
            32772=> self.r4,
            32773=> self.r5,
            32774=> self.r6,
            32775=> self.r7,
            _ => panic!("Unknown register, {}", register),
        }
    }

    pub fn set_reg(&mut self, register:u16, value: u16) {
        // println!("Setting reg {} to val {}", register, value);
        match register {
            32768 => self.r0 = value,
            32769 => self.r1 = value,
            32770 => self.r2 = value,
            32771=> self.r3 = value,
            32772=> self.r4 = value,
            32773=> self.r5 = value,
            32774=> self.r6 = value,
            32775=> self.r7 = value,
            _ => panic!("Unknown register"),
        };
    }
    
    fn parse_number(&mut self, number: u16) -> u16 {
        match number {
            0...32767 => {
                // println!("literal");
                number
            },
            32768...32775 => {
                // println!("reg");
                self.get_reg(number)
            },
            32776...65535 => {
                panic!("invalid");
            },
            _ => panic!("Invalid"),
        }
    }
    
    pub fn cycle(&mut self) {
        self.opcode = self.get_opcode();
        // println!("{}", Opcode::parse(self.opcode, self.pc));
        match Opcode::parse(self.opcode, self.pc) {
            Opcode::Noop => {
                self.pc += 1;
            },
            Opcode::Out => {
                let value = self.read_location(1) as u8;
                print!("{}", value as char);
                self.pc += 2;
            },
            Opcode::In => {
                // let location = self.read_location(1);
                let location = self.memory[(self.pc + 1) as usize];
                match io::stdin().bytes().next() {
                    Some(Ok(val)) =>  {
                        self.set_reg(location, val as u16);
                        // self.memory[location as usize] = val as u16;
                    },
                    Some(Err(e))  => panic!(e),
                    None => panic!(),
                };
                self.pc += 2;
            },
            Opcode::Jmp => {
                let location = self.read_location(1); 
                self.pc = location;
            },
            Opcode::Jt => {
                let bool = self.read_location(1);
                self.parse_number(bool);
                if bool != 0 {
                    self.pc = self.read_location(2);
                } else {
                    self.pc += 3;
                }
            },
            Opcode::Jf => {
                let bool = self.read_location(1);
                self.parse_number(bool);
                if bool == 0 {
                    self.pc = self.read_location(2);
                } else {
                    self.pc += 3;
                }
            },
            Opcode::Set => {
                let number = self.memory[(self.pc + 1) as usize];
                let value = self.read_location(2);
                self.set_reg(number, value);
                self.pc += 3;
            },
            Opcode::Add => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                self.set_reg(number, (b + c) % 32768);
                self.pc += 4;
            },
            Opcode::Mult => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                self.set_reg(number, (b.wrapping_mul(c)) % 32768);
                self.pc += 4;
            },
            Opcode::Mod => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                self.set_reg(number, (b % c));
                self.pc += 4;
            },
            Opcode::Eq => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                if b == c {
                    self.set_reg(number, 1);
                } else {
                    self.set_reg(number, 0);
                }
                self.pc += 4;
            },
            Opcode::Push => {
                let a = self.read_location(1);
                self.stack.push(a);
                self.pc += 2;
            },
            Opcode::Rmem => {
                let reg = self.memory[(self.pc + 1) as usize];
                let location = self.read_location(2);
                let value = self.memory[location as usize];
                self.set_reg(reg, value);
                self.pc += 3;
            },
            Opcode::Wmem => {
                let a = self.read_location(1);
                let b = self.read_location(2);
                self.memory[a as usize] = b;
                self.pc += 3;
            },
            Opcode::Pop => {
                let value = match self.stack.pop() {
                    Some(value) => value,
                    None => panic!("err"),
                };
                let reg = self.memory[(self.pc + 1) as usize];
                self.set_reg(reg, value);
                    self.pc += 2;
            },
            Opcode::Gt => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                if b > c {
                    self.set_reg(number, 1);
                } else {
                    self.set_reg(number, 0);
                }
                self.pc += 4;
            },
            Opcode::And => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                self.set_reg(number, (b & c));
                self.pc += 4;
            },
            Opcode::Or => {
                let number = self.memory[(self.pc + 1) as usize];
                let b = self.read_location(2);
                let c = self.read_location(3);
                self.set_reg(number, (b | c));
                self.pc += 4;
            },
            Opcode::Not => {
                let reg = self.memory[(self.pc + 1) as usize];
                let number = self.read_location(2);
                self.set_reg(reg, !number & 0x7fff);
                self.pc += 3;
            },
            Opcode::Call => {
                // let next = self.memory[(self.pc + 2) as usize];
                self.stack.push(self.pc + 2);
                let call = self.read_location(1);
                self.pc = call;
            },
            Opcode::Ret => {
                let value = match self.stack.pop() {
                    Some(val) => val,
                    None => panic!("No stack")
                };

                self.pc = value;
            },
            Opcode::Halt => {
                exit(0);
            }
            _ => panic!("Got code, {}", Opcode::parse(self.opcode, self.pc)),
        };
    }
    
}
