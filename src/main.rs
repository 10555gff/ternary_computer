use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;

fn main() {
 //   let a = trits!("T01_0");
    let b = trits!("00_00");
    let c = trits!("0001");
    let d = trits!("0011");

    // let code=9;
    
    // let result1 =a.gate_core(b,code);
    // let result2 =a.gate_core(c,code);
    // let result3 =a.gate_core(d,code);

// let x = c.set(1,0b11);
// println!("{}",x);
    let result1 = c.adder(d,0);
    // let result3 = a.tsum(d);
    // println!("{}",a);

    println!("CARRY:{}",result1.0);
    println!("SUM  :{}",result1.1);
// c.set(0,0b11);
//     println!("{}",c);
    // println!("结果  : {:08b}", result1.0);
    // println!("结果  : {:08b}", result2.0);
    // println!("结果  : {:08b}", result3.0);
}

