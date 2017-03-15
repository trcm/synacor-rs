use std::fmt;

const OPCODE_HALT: u16 = 0;
const OPCODE_SET: u16  = 1;// a b
const OPCODE_PUSH: u16 = 2;// a
const OPCODE_POP: u16  = 3;// a
const OPCODE_EQ: u16   = 4;// a b c
const OPCODE_GT: u16   = 5;// a b c
const OPCODE_JMP: u16  = 6;// a
const OPCODE_JT: u16   = 7;// a b
const OPCODE_JF: u16   = 8;// a b
const OPCODE_ADD: u16  = 9;// a b c
const OPCODE_MULT: u16 = 10;// a b c
const OPCODE_MOD: u16  = 11;// a b c
const OPCODE_AND: u16  = 12;// a b c
const OPCODE_OR: u16   = 13;// a b c
const OPCODE_NOT: u16  = 14;// a b
const OPCODE_RMEM: u16 = 15;// a b
const OPCODE_WMEM: u16 = 16;// a b
const OPCODE_CAL: u16  = 17;// a
const OPCODE_RET: u16  = 18;
const OPCODE_OUT: u16  = 19;// a
const OPCODE_IN: u16   = 20;// a
const OPCODE_NOOP: u16 = 21;

pub enum Opcode {
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    Noop,
}

impl Opcode {
    pub fn parse(num: u16, pc: u16) -> Opcode {
        match num {
            OPCODE_HALT => Opcode::Halt,
            OPCODE_SET  => Opcode::Set,
            OPCODE_PUSH => Opcode::Push,
            OPCODE_POP  => Opcode::Pop,
            OPCODE_EQ   => Opcode::Eq,
            OPCODE_GT   => Opcode::Gt,
            OPCODE_JMP  => Opcode::Jmp,
            OPCODE_JT   => Opcode::Jt,
            OPCODE_JF   => Opcode::Jf,
            OPCODE_ADD  => Opcode::Add,
            OPCODE_MULT => Opcode::Mult,
            OPCODE_MOD  => Opcode::Mod,
            OPCODE_AND  => Opcode::And,
            OPCODE_OR   => Opcode::Or,
            OPCODE_NOT  => Opcode::Not,
            OPCODE_RMEM => Opcode::Rmem,
            OPCODE_WMEM => Opcode::Wmem,
            OPCODE_CAL  => Opcode::Call,
            OPCODE_RET  => Opcode::Ret,
            OPCODE_OUT  => Opcode::Out,
            OPCODE_IN   => Opcode::In,
            OPCODE_NOOP => Opcode::Noop,
            _ => panic!("Oops")
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            &Opcode::Halt => "HALT",
            &Opcode::Set  => "SET a b",
            &Opcode::Push => "PUSH a",
            &Opcode::Pop  => "POP a",
            &Opcode::Eq   => "EQ a b c",
            &Opcode::Gt   => "GT a b c",
            &Opcode::Jmp  => "JMP a",
            &Opcode::Jt   => "JT a b",
            &Opcode::Jf   => "JF a b",
            &Opcode::Add  => "ADD a b c",
            &Opcode::Mult => "MULT a b c",
            &Opcode::Mod  => "MOD a b c",
            &Opcode::And  => "AND a b c",
            &Opcode::Or   => "OR a b c",
            &Opcode::Not  => "NOT a b",
            &Opcode::Rmem => "RMEM a b",
            &Opcode::Wmem => "WMEM a b",
            &Opcode::Call => "CALL a",
            &Opcode::Ret  => "RET",
            &Opcode::Out  => "OUT a",
            &Opcode::In   => "IN a",
            &Opcode::Noop => "NOOP",
        };
        write!(f, "{}", code)
    }
}
