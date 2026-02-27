use core::cmp::Ordering;
use crate::ternary_cpu::logical_alu::Trit4;

const INST_SIZE: usize = 3;
const INVALID: u8 = 0xFF;
const DECODE_LUT: [u8; 16] = [
    4,      // 0x00
    5,      // 0x01
    3,      // 0x02
    INVALID,// 0x03
    7,      // 0x04
    8,      // 0x05
    6,      // 0x06
    INVALID,// 0x07
    1,      // 0x08
    2,      // 0x09
    0,      // 0x0A
    INVALID,// 0x0B
    INVALID,// 0x0C
    INVALID,// 0x0D
    INVALID,// 0x0E
    INVALID,// 0x0F
];

#[derive(Debug,Clone, Copy)]
pub struct Register {
    pub regs: [Trit4; 9], // R0..R8
}

enum Instruction {
    Imm { reg: usize, val: Trit4 },
    Copy { src: usize, dst: usize },
    Calc { src: usize, code: u8 },
    Condition { jump_type: u8, val :u8 },
    Halt,
    Unknown,
}

pub struct Flags {
    pub zero: bool,
    pub negative: bool,
    pub positive: bool,
}

pub struct T80CPU {
    pub pc: usize,
    pub mem: Vec<u8>,
    pub regs: Register,
    pub flags: Flags,   // ← 加这里
    pub halted: bool,
}


impl Flags {
    pub fn new() -> Self {
        Self {
            zero: false,
            negative: false,
            positive: false,
        }
    }

    // 修正：去掉多餘的 self.flags
    pub fn update_flags(&mut self, val: Trit4) {
        let cmp = val.cmp(&Trit4::ZERO);
        self.zero     = cmp == Ordering::Equal;
        self.negative = cmp == Ordering::Less;
        self.positive = cmp == Ordering::Greater;
    }
}

impl Register {
    pub fn new() -> Self {
        Self { regs: [Trit4(0); 9] }
    }

    pub fn read(&self, idx: usize) -> Trit4 {
        self.regs[idx]
    }

    pub fn write(&mut self, idx: usize, val: Trit4) {
        self.regs[idx] =val;
    }

    pub fn write_u8(&mut self, idx: usize, val: u8) {
        self.regs[idx] =Trit4(val);
    }
}


//CPU Fetch → Decode → Execute
impl T80CPU {
    pub fn new(mem: Vec<u8>) -> Self {
        Self {
            pc: 0,
            mem,
            flags: Flags::new(),
            regs: Register::new(),
            halted: false,
        }
    }
    fn decode_address(byte: u8) -> u8 {
        let val = DECODE_LUT[byte as usize];
        if val == 0xFF {
            panic!("invalid address byte: {:#04X}", byte);
        }
        val
    }


    //从 PC 取指令，并推进 PC
    pub fn fetch(&mut self) -> Option<[u8; 3]> {
        let end = self.pc.checked_add(INST_SIZE)?;
        if end > self.mem.len() {
            return None;
        }
        let inst = [
            self.mem[self.pc],
            self.mem[self.pc + 1],
            self.mem[self.pc + 2],
        ];
        self.pc = end;
        Some(inst)
    }

    //解析 opcode → 结构化指令
    fn decode(&self, inst: [u8; 3]) -> Instruction {
        match inst[0] {
            0x00 => Instruction::Imm {
                reg: 0,
                val: Trit4(inst[2]),
            },

            0x10 => Instruction::Copy {
                src: Self::decode_address(inst[1]) as usize,
                dst: Self::decode_address(inst[2]) as usize,
            },

            0x60 => Instruction::Calc {
                src: Self::decode_address(inst[1]) as usize,
                code: Self::decode_address(inst[2]),
            },

            0x40 => Instruction::Condition { 
                jump_type: Self::decode_address(inst[2]),
                val:Self::decode_address(self.regs.read(0).0),
            },
            
            0xFF => Instruction::Halt,

            _ => Instruction::Unknown,
        }
    }

    //执行指令
    fn execute(&mut self, inst: Instruction) {
        match inst {

            Instruction::Imm { reg, val } => {
                self.regs.write(reg, val);
            }

            Instruction::Copy { src, dst } => {
                let val = self.regs.read(src);
                self.regs.write(dst, val);
            }

            Instruction::Calc { src, code } => {
                let a = self.regs.read(src);
                let b = self.regs.read(6);
                let res = b.gate_core(a, code);
                self.regs.write(8, res);
                //self.flags.update_flags(res);   // ← 建議加上
            }

            Instruction::Condition { jump_type,val } => {
                //let reg3 = self.regs.read(3);
                let should_jump = match jump_type {
                    0 => false,                             // never
                    1 => self.flags.zero,                   // z
                    2 => self.flags.zero || self.flags.negative,  // <= (z or n)
                    3 => self.flags.negative,               // n
                    4 => true,                              // always
                    5 => self.flags.positive,               // p
                    6 => self.flags.zero || self.flags.positive,  // >= (z or p)
                    7 => !self.flags.zero,                  // nz
                    _ => false,
                };

                if should_jump{
                    self.pc = (val *3) as usize;   // ← 这里覆盖 PC
                }

            }

            Instruction::Halt => {
                self.halted = true;
            }

            Instruction::Unknown => {
                panic!("Illegal opcode at PC={}", self.pc - 3);
                //println!("Unknown opcode {:X}", opcode),
            }
        }
    }

    pub fn step(&mut self) -> bool {
        match self.fetch() {
            Some(raw) => {
                let inst = self.decode(raw);
                self.execute(inst);
                true
            }
            None => false
        }
    }


    //控制整个循环
    pub fn run(&mut self) {
        while !self.halted && self.step() {}
        if self.halted {
            println!("CPU 已 halt");
        }
    }

}
