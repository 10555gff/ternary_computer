use crate::ternary_cpu::logical_alu::Trit4;

#[derive(Debug,Clone, Copy)]
pub struct Register {
    pub regs: [Trit4; 9], // R0..R8
}

impl Register {
    pub fn new() -> Self {
        Self { regs: [Trit4(0); 9] }
    }

    pub fn read(&self, idx: usize) -> Trit4 {
        self.regs[idx]
    }

    pub fn write(&mut self, idx: usize, val: u8) {
        self.regs[idx] = Trit4(val);
    }
}

// struct ROM {
//     data: Vec<u8>,
// }

// struct RAM {
//     data: Vec<u8>,
// }

pub struct T80CPU {
    pub pc: usize,
    pub mem: Vec<u8>,
    pub regs: Register,
    pub halted: bool,
}

impl T80CPU {
    pub fn new(mem: Vec<u8>) -> Self {
        Self {
            pc: 0,
            mem,
            regs: Register::new(),
            halted: false,
        }
    }
    pub fn fetch(&mut self) -> [u8; 3] {
        if (self.pc + 3) > self.mem.len() {
            panic!("Fetch beyond memory");
        }
        let byte1 = self.mem[self.pc];
        let byte2 = self.mem[self.pc + 1];
        let byte3 = self.mem[self.pc + 2];
        self.pc += 3;
        [byte1, byte2, byte3]
    }

    pub fn run(&mut self) {
        while !self.halted && self.pc + 3 <= self.mem.len() {
            let inst_bytes = self.fetch();
            println!("BBBBBBBBBBBBBBBBBBBBB         {:08b} {:08b} {:08b}",inst_bytes[0],inst_bytes[1],inst_bytes[2]);
            self.decode_execute(inst_bytes);
        }
        if self.halted {
            println!("CPU 已 halt");
        } else if self.pc >= self.mem.len() {
            println!("PC 超出内存，程序自然结束");
        }
    }


    fn decode_address(byte: u8) -> u8 {
        match byte {
            0x0A => 0,
            0x08 => 1,
            0x09 => 2,
            0x02 => 3,
            0x00 => 4,
            0x01 => 5,
            0x06 => 6,
            0x04 => 7,
            0x05 => 8,
            _ => panic!("invalid address byte: {:#04X}", byte),
        }
    }


    fn decode_execute(&mut self, inst: [u8; 3]) {
        let opcode = inst[0];
        
        match opcode {
            0x00 => {//Immediate,00
                self.regs.write(0, inst[2]);
            },
            0x10 => {//Copy,01
                let src  = Self::decode_address(inst[1])  as usize;
                let dest = Self::decode_address(inst[2])  as usize;

                let val = self.regs.read(src);
                self.regs.write(dest, val.0);
            },
            0x60 => println!("C"),//Calculate,
            0x40 => println!("D"),//Condition,
            _ => println!("Unknown opcode {:X}", opcode),
        }

        // match opcode {
        //     1 => { // Copy

        //         if src > 7 || dest > 7 {
        //             panic!("Invalid register index");
        //         }
        //         Instruction::Copy { src, dest }
        //     }
        //     2 => { // Condition (Le)
        //         let src1 = (inst[1] >> 4) as usize;
        //         let src2 = (inst[1] & 0x0F) as usize;
        //         let offset = inst[2] as i8;
        //         if src1 > 7 || src2 > 7 {
        //             panic!("Invalid register index");
        //         }
        //         Instruction::Condition { src1, src2, offset }
        //     }
        //     _ => panic!("Unknown opcode: {}", opcode),
        // }
    }
    // fn execute(&mut self, inst: Instruction) {
    //         match inst {
    //             Instruction::Copy { src, dest } => {
    //                 let val = self.regs.read(src);
    //                 self.regs.write(dest, val);
    //             }
    //             Instruction::Condition { src1, src2, offset } => {
    //                 let a = self.regs.read(src1);
    //                 let b = self.regs.read(src2);
    //                 if a <= b {
    //                     // Jump relative
    //                     let jump_bytes = (offset as isize) * 3;
    //                     if jump_bytes < 0 && (jump_bytes.abs() as usize) > self.pc {
    //                         panic!("Jump before start");
    //                     }
    //                     self.pc = ((self.pc as isize) + jump_bytes) as usize;
    //                 }
    //                 // Else continue
    //             }
    //         }
    //     }


//     #[derive(Debug)]
// enum Instruction {
//     Copy { src: usize, dest: usize },
//     Condition { src1: usize, src2: usize, offset: i8 }, // Relative offset in instructions
//     // Add Immediate and Calculate if needed in future
// }
    // fn nop(&self) {}

    // fn halt(&self) {
    //     println!("HALT");
    //     std::process::exit(0);
    // }

    // fn load_imm(&mut self, reg: u8) {
    //     let imm = self.fetch();
    //     self.regs.write(reg as usize, Trit4(imm));
    // }

    // fn add(&mut self, reg: u8) {
    //     let a = self.regs.read(0);
    //     let b = self.regs.read(reg as usize);
    //     self.regs.write(0, a + b);
    // }


}














































// use trit_macro::trits;
// use ternary_arithmetic::ternary_cpu::trits::Trit4;

// #[derive(Clone)]
// pub struct Ternary {
//     pub digits: Vec<Trit>,
// }









// pub struct PC {
//     pub value: Ternary,
// }

// impl PC {
//     pub fn new() -> Self {
//         Self { value: Ternary { digits: vec![] } }
//     }

//     pub fn inc(&mut self) {
//         // balanced ternary increment
//     }
// }



// pub struct ALU;

// impl ALU {
//     pub fn add(a: &Ternary, b: &Ternary) -> Ternary {
//         // 你已有 ternary_stack_adder
//         todo!()
//     }

//     pub fn sub(a: &Ternary, b: &Ternary) -> Ternary {
//         todo!()
//     }

//     pub fn mul(a: &Ternary, b: &Ternary) -> Ternary {
//         todo!()
//     }
// }





// // memory.rs
// pub struct ROM {
//     pub data: Vec<u8>,
// }

// pub struct RAM {
//     pub data: Vec<u8>,
// }

// pub struct Memory {
//     pub rom: ROM,
//     pub ram: RAM,
// }

// impl Memory {
//     pub fn read_byte(&self, addr: usize) -> u8 {
//         if addr < 0x4000 {
//             self.rom.data[addr]
//         } else {
//             self.ram.data[addr - 0x4000]
//         }
//     }

//     pub fn write_byte(&mut self, addr: usize, val: u8) {
//         if addr >= 0x4000 {
//             self.ram.data[addr - 0x4000] = val;
//         }
//     }
// }





// // bus.rs
// use crate::memory::Memory;

// pub struct Bus {
//     pub mem: Memory,
// }

// impl Bus {
//     pub fn read(&self, addr: usize) -> u8 {
//         self.mem.read_byte(addr)
//     }

//     pub fn write(&mut self, addr: usize, val: u8) {
//         self.mem.write_byte(addr, val);
//     }
// }






// // cpu.rs
// use crate::{pc::PC, register::RegisterFile, alu::ALU, bus::Bus, isa::*};


// impl CPU {
//     pub fn new(bus: Bus) -> Self {
//         Self {
//             pc: PC::new(),
//             regs: RegisterFile::new(),
//             alu: ALU,
//             bus,
//         }
//     }

//     fn fetch(&mut self) -> u8 {
//         let addr = self.pc.value.to_usize(); // ternary → usize
//         let byte = self.bus.read(addr);
//         self.pc.inc();
//         byte
//     }

//     fn execute(&mut self, opcode: Opcode, arg: u8) {
//         match opcode {
//             Opcode::Load => {
//                 let imm = self.fetch();
//                 self.regs.write(arg as usize, imm.into());
//             }
//             Opcode::Add => {
//                 let a = self.regs.read(0).clone();
//                 let b = self.regs.read(arg as usize);
//                 self.regs.write(0, ALU::add(&a, b));
//             }
//             Opcode::Halt => {
//                 println!("HALT");
//                 std::process::exit(0);
//             }
//             _ => {}
//         }
//     }

//     pub fn run(&mut self) {
//         loop {
//             let byte = self.fetch();
//             let opcode = decode(byte);
//             let arg = byte & 0x0F;
//             self.execute(opcode, arg);
//         }
//     }
// }









// impl ROM {
//     fn read(&self, addr: usize) -> u8 {
//         self.data[addr]
//     }
// }

// impl RAM {
//     fn read(&self, addr: usize) -> u8 {
//         self.data[addr]
//     }

//     fn write(&mut self, addr: usize, val: u8) {
//         self.data[addr] = val;
//     }
// }

// //CPU Fetch → Decode → Execute
// impl T80CPU {
//     fn fetch(&mut self) -> u8 {
//         let byte = self.mem[self.pc];
//         self.pc += 1;
//         byte
//     }

//     fn nop(&self) {}

//     fn halt(&self) {
//         println!("HALT");
//         std::process::exit(0);
//     }

//     fn load_imm(&mut self, reg_id: u8) {
//         let imm = self.fetch();
//         self.regs[reg_id as usize].trits[0] = Trit4(imm);
//     }

//     fn add(&mut self, reg_id: u8) {
//         // R0 = R0 + Rn
//         let a = self.regs[0].trits[0];
//         let b = self.regs[reg_id as usize].trits[0];
//         self.regs[0].trits[0] = a + b; // 你已实现 ternary Add
//     }

//     fn run(&mut self) {
//         loop {
//             let byte = self.fetch();
//             self.decode_execute(byte);
//         }
//     }

// }





// // fn CPU(){
// //     let r0;
// //     let r1;
// //     let pc=0;
// // }


// // // this.run=fn(){
// // //     while(true){
// // //         const byte=read_byte(pc);
// // //         decodeExecute(byte);
// // //         pc++;
// // //     }
// // // }

// // // const cpu=new CPU();
// // // const rom=new ROM();
// // // const ram=new RAM();


// // // fn read_byte(address){
// // //     if(address<0x4000){
// // //         return rom.read(address);
// // //     }else{
// // //         read ram.read(address - 0X4000);
// // //     }
// // // }

// // loop {
// //     let byte = cpu.fetch();
// //     cpu.decode_execute(byte);
// // }


// // main.rs
// mod ternary;
// mod pc;
// mod register;
// mod alu;
// mod memory;
// mod bus;
// mod isa;
// mod cpu;

// use bus::*;
// use memory::*;
// use cpu::*;

// fn main() {
//     let bus = Bus {
//         mem: Memory {
//             rom: ROM { data: vec![0x10,0x05,0xF0] },
//             ram: RAM { data: vec![0; 0x4000] },
//         }
//     };

//     let mut cpu = CPU::new(bus);
//     cpu.run();
// }