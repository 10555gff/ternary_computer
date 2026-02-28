use std::cmp::Ordering;
use crate::ternary_cpu::logical_alu::Trit4;

const INST_SIZE: usize = 3;
const DECODE_LUT: [u8; 16] = [
    4,      // 0x00
    5,      // 0x01
    3,      // 0x02
    0xFF,   // 0x03
    7,      // 0x04
    8,      // 0x05
    6,      // 0x06
    0xFF,   // 0x07
    1,      // 0x08
    2,      // 0x09
    0,      // 0x0A
    0xFF,   // 0x0B
    0xFF,   // 0x0C
    0xFF,   // 0x0D
    0xFF,   // 0x0E
    0xFF,   // 0x0F
];

#[derive(Debug,Clone, Copy)]
pub struct Register {
    pub regs: [Trit4; 9], // R0..R8
}

enum Instruction {
    Imm  { val: Trit4 },
    Copy { src: usize, dst: usize },
    Calc { src: usize, code: u8 },
    Condition { jump_type: u8, val :u8 },
    Halt,
    Unknown,
}

pub struct T80CPU {
    pub pc: usize,
    pub mem: Vec<u8>,
    pub regs: Register,
    pub halted: bool,
}


impl Register {
    pub fn new() -> Self {
        Self { regs: [Trit4(0); 9] }
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
            regs: Register::new(),
            halted: false,
        }
    }
    fn decode_address(byte: u8) -> u8 {
        DECODE_LUT
            .get(byte as usize)
            .copied()
            .unwrap_or(0xFF)
    }
    fn check_condition(&self, jump_type: u8) -> bool {
        let cmp = self.regs[8].cmp(&Trit4::ZERO);
        match jump_type {
            0 => false,
            1 => cmp == Ordering::Equal,
            2 => cmp != Ordering::Greater,
            3 => cmp == Ordering::Less,
            4 => true,
            5 => cmp == Ordering::Greater,
            6 => cmp != Ordering::Less,
            7 => cmp != Ordering::Equal,
            _ => false,
        }
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
                val: Self::decode_address(inst[1]),
                jump_type: Self::decode_address(inst[2]),
            },
            
            0xFF => Instruction::Halt,

            _ => Instruction::Unknown,
        }
    }

    //执行指令
    fn execute(&mut self, inst: Instruction) {
        match inst {

            Instruction::Imm { val } => {
                self.regs[0]=val;
            }

            Instruction::Copy { src, dst } => {
                self.regs[dst]= self.regs[src];
            }

            Instruction::Calc { src, code } => {
                let a = self.regs[src];
                let b = self.regs[7];
                let res = b.gate_core(a, code);

                self.regs[8]=res;
            }

            Instruction::Condition { jump_type,val } => {
                if self.check_condition(jump_type) {
                    //执行→ 改 PC → 再执行 → 再改 PC
                    //PC = loop地址
                    self.pc = (val as usize) * INST_SIZE;
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


impl std::ops::Index<usize> for Register {
    type Output = Trit4;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.regs[idx]
    }
}

impl std::ops::IndexMut<usize> for Register {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.regs[idx]
    }
}
