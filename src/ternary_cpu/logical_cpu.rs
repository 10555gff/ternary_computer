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
    Imm  { dst: usize,val: Trit4 },
    Copy { src: usize, dst: usize },
    Calc { src: usize, ctype: u8 },
    Condition { jump_type: u8, offset: i8},
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
        let cmp = self.regs[3].cmp(&Trit4::ZERO);
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

    pub fn fetch(&mut self) -> Option<[u8; INST_SIZE]> {
        let bytes = self.mem.get(self.pc..self.pc + INST_SIZE)?;
        self.pc += INST_SIZE;
        Some([bytes[0], bytes[1], bytes[2]])
    }

    //解析 opcode → 结构化指令
    fn decode(&self, inst: [u8; 3]) -> Instruction {
        //let opcode = inst[0] & 0xF0;
        match inst[0] {
            //立即数模式，val 是直接加载到寄存器的 Trit4 值
            0x00 => Instruction::Imm {
                dst: Self::decode_address(inst[1]) as usize,
                val: Trit4(inst[2]),
            },
            //复制模式，src 和 dst 分别表示源寄存器和目标寄存器的索引
            0x10 => Instruction::Copy {
                src: Self::decode_address(inst[1]) as usize,
                dst: Self::decode_address(inst[2]) as usize,
            },
            //计算模式，src 表示源寄存器索引，ctype 表示运算类型
            0x60 => Instruction::Calc {
                src: Self::decode_address(inst[1]) as usize,
                ctype: Self::decode_address(inst[2]),
            },
            //条件跳转模式，根据 jump_type 和 reg3寄存器值 决定是否跳转
            0x40 => Instruction::Condition {
                jump_type: Self::decode_address(inst[1]),
                offset: Trit4(inst[2]).to_dec(),
            },
            //停止指令，表示 CPU 执行停止
            0xFF => Instruction::Halt,

            _ => Instruction::Unknown,
        }
    }

    //执行指令
    fn execute(&mut self, inst: Instruction) {
        match inst {

            Instruction::Imm { dst, val } => {
                self.regs[dst]=val;
            }

            Instruction::Copy { src, dst } => {
                self.regs[dst]= self.regs[src];
            }

            Instruction::Calc { src, ctype } => {
                let a = self.regs[src];
                let b = self.regs[0];
                let res = b.gate_core(a, ctype);

                self.regs[src]=res;
            }

            Instruction::Condition { jump_type,offset } => {
                if self.check_condition(jump_type) {
                    let jump_bytes = offset as isize * INST_SIZE as isize;
                    self.pc = ((self.pc as isize) + jump_bytes) as usize;
                }
            }

            Instruction::Halt => {
                self.halted = true;
            }

            Instruction::Unknown => {
                self.halted = true;
                eprintln!("Unknown opcode at PC={}", self.pc - INST_SIZE);
            }
        }
    }

    pub fn step(&mut self) {
        match self.fetch() {
            Some(raw) => {
                let inst = self.decode(raw);
                self.execute(inst);
            }
            None => {
                self.halted = true;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
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
