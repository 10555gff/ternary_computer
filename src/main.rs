use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;

fn main() {
    let rom = vec![
        0x10, 0x05, // LOAD R0, 5
        0x11, 0x03, // LOAD R1, 3
        0x21,       // ADD R1
        0xF0,       // HALT
    ];

    let mut cpu = T80CPU {
        pc: 0,
        mem: rom,
        regs: [Register { trits: [Trit4(0); 8] }; 8],
    };

    cpu.run();


    // let a = trits!("0011"); // 1
    // let b = trits!("0001"); // 1
    // let c = trits!("0001"); // 1

    // // 第一步：用 adder 初始化第一个 TritResult
    // // 假设初始进位是 0
    // // let res = a.adder(b, 2); 

    // // 第二步：直接用 TritResult + Trit4
    // // 此时它内部会自动执行：c.adder(res.sum, res.carry)
    // let final_res = a+b ; 

    // println!("{}", final_res);














    // let a = trits!("T01_0");
    // let b = trits!("---0");
    // let c = trits!("0000");
    // let d = trits!("+++0");
    // let z =0b01;

    // let code=9;
    
    // let result1 =a.gate_core(b,code);
    // let result2 =a.gate_core(c,code);
    // let result3 =a.gate_core(d,code);

// let x = c.set(1,0b11);
// println!("{}",x);


    // let result2=  a ^ c;
    // let result3=  a ^ d;
    // let result3 = a.tsum(d);
    // println!("{}",a);

    // println!("CARRY:{}",result1.0);
    // println!("SUM  :{}",result1.1);
// c.set(0,0b11);

    // println!("{}",result2);
    // println!("{}",result3);

    // println!("{}",a);

    // println!("结果  : {:08b}", result1.0);
    // println!("结果  : {:08b}", result2.0);
    // println!("结果  : {:08b}", result3.0);
}

