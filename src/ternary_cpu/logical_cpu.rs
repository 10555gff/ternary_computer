use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;

// 32 trits = 8 × Trit4
#[derive(Clone)]
struct Register {
    trits: [Trit4; 8],
}

struct ROM {
    data: Vec<u8>,
}

struct RAM {
    data: Vec<u8>,
}

struct T80CPU {
    pc: usize,      // byte PC
    mem: Vec<u8>,   // unified memory (ROM+RAM mapped)
    regs: [Register; 8], // R0..R7
}



impl ROM {
    fn read(&self, addr: usize) -> u8 {
        self.data[addr]
    }
}

impl RAM {
    fn read(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }
}

//CPU Fetch → Decode → Execute
impl T80CPU {
    fn fetch(&mut self) -> u8 {
        let byte = self.mem[self.pc];
        self.pc += 1;
        byte
    }

    fn nop(&self) {}

    fn halt(&self) {
        println!("HALT");
        std::process::exit(0);
    }

    fn load_imm(&mut self, reg_id: u8) {
        let imm = self.fetch();
        self.regs[reg_id as usize].trits[0] = Trit4(imm);
    }

    fn add(&mut self, reg_id: u8) {
        // R0 = R0 + Rn
        let a = self.regs[0].trits[0];
        let b = self.regs[reg_id as usize].trits[0];
        self.regs[0].trits[0] = a + b; // 你已实现 ternary Add
    }

    fn run(&mut self) {
        loop {
            let byte = self.fetch();
            self.decode_execute(byte);
        }
    }

}


fn decode_execute(&mut self, byte: u8) {
    let opcode = byte >> 4;
    let arg = byte & 0x0F;

    match opcode {
        0x0 => self.nop(),
        0x1 => self.load_imm(arg),
        0x2 => self.add(arg),
        0xF => self.halt(),
        _ => println!("Unknown opcode {:X}", opcode),
    }
}


// fn CPU(){
//     let r0;
//     let r1;
//     let pc=0;
// }


// // this.run=fn(){
// //     while(true){
// //         const byte=read_byte(pc);
// //         decodeExecute(byte);
// //         pc++;
// //     }
// // }

// // const cpu=new CPU();
// // const rom=new ROM();
// // const ram=new RAM();


// // fn read_byte(address){
// //     if(address<0x4000){
// //         return rom.read(address);
// //     }else{
// //         read ram.read(address - 0X4000);
// //     }
// // }