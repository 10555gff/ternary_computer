use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;

fn main() {
    let a = trits!("T01_0");
    let b = trits!("TTT_0");
    let c = trits!("000_0");
    let d = trits!("111_0");

    let code=8;
    
    let result1 =a.gate_core(b,code);
    let result2 =a.gate_core(c,code);
    let result3 =a.gate_core(d,code);

    // let result1 = a.tany(b);
    // let result2 = a.tany(c);
    // let result3 = a.tany(d);
    // println!("{}",a);

    println!("{}",result1);
    println!("{}",result2);
    println!("{}",result3);
    // println!("结果  : {:08b}", result1.0);
    // println!("结果  : {:08b}", result2.0);
    // println!("结果  : {:08b}", result3.0);
}

