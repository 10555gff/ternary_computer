use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {
    let a = trits!("T01_0");
    let b = trits!("---0");
    let c = trits!("0000");
    let d = trits!("+++0");

    let code=9;
    
    let result1 =a.gate_core(b,code);
    let result2 =a.gate_core(c,code);
    let result3 =a.gate_core(d,code);


    println!("{}",result1);
    println!("{}",result2);
    println!("{}",result3);


    // let rom = vec![
    //     0x10, 0x05, // LOAD R0, 5
    //     0x11, 0x03, // LOAD R1, 3
    //     0x21,       // ADD R1
    //     0xF0,       // HALT
    // ];

    // let mut cpu = T80CPU {
    //     pc: 0,
    //     mem: rom,
    //     regs: [Register { trits: [Trit4(0); 8] }; 8],
    // };

    // cpu.run();



    //     let bus = Bus {
    //     mem: Memory {
    //         rom: ROM { data: vec![0x10,0x05,0xF0] },
    //         ram: RAM { data: vec![0; 0x4000] },
    //     }
    // };

    // let mut cpu = CPU::new(bus);
}