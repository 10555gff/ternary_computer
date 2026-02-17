use ternary_arithmetic::ternary_cpu::trit::Trit4;

fn main() {
    let a = Trit4(0b10_00_01_00); 
    let b = Trit4(0b10_10_10_00);
    let c = Trit4(0b00_00_00_00);
    let d = Trit4(0b01_01_01_00);
    // let code=3;
    
    // let r1=a.gate_core(b,0);
    // let r2=a.gate_core(b,3);
    // let result1 =r1.gate_core(r2,3);





    let result1 = a.txor(b);
    let result2 = a.txor(c);
    let result3 = a.txor(d);


    println!("结果  : {:08b}", result1.0);
    println!("结果  : {:08b}", result2.0);
    println!("结果  : {:08b}", result3.0);
}