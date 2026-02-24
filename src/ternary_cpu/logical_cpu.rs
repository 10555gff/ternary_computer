use crate::ternary_cpu::logical_alu::Trit4;


#[derive(Clone, Copy)]
pub struct Register {
    pub regs: [Trit4; 8], // R0..R7
}

impl Register {
    pub fn new() -> Self {
        Self { regs: [Trit4(0); 8] }
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





// // isa.rs
// pub enum Opcode {
//     Nop,
//     Load,
//     Add,
//     Sub,
//     Halt,
// }

// pub fn decode(byte: u8) -> Opcode {
//     match byte >> 4 {
//         0x0 => Opcode::Nop,
//         0x1 => Opcode::Load,
//         0x2 => Opcode::Add,
//         0x3 => Opcode::Sub,
//         0xF => Opcode::Halt,
//         _ => Opcode::Nop,
//     }
// }




// // cpu.rs
// use crate::{pc::PC, register::RegisterFile, alu::ALU, bus::Bus, isa::*};

// pub struct CPU {
//     pub pc: PC,
//     pub regs: RegisterFile,
//     pub alu: ALU,
//     pub bus: Bus,
// }

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





// struct T80CPU {
//     pc: usize,      // byte PC
//     mem: Vec<u8>,   // unified memory (ROM+RAM mapped)
//     regs: [Register; 8], // R0..R7
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


// fn decode_execute(&mut self, byte: u8) {
//     let opcode = byte >> 4;
//     let arg = byte & 0x0F;

//     match opcode {
//         0x0 => self.nop(),
//         0x1 => self.load_imm(arg),
//         0x2 => self.add(arg),
//         0xF => self.halt(),
//         _ => println!("Unknown opcode {:X}", opcode),
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