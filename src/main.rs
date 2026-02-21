use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;

fn main() {

    let c = trits!("0011");
    let d = trits!("1111");

    let result1 = c.adder(d,0);


    println!("CARRY:{}",result1.carry);
    println!("SUM  :{}",result1.sum);















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

