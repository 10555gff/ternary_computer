use ternary_arithmetic::ternary_cpu::trit::Trit4;

fn main() {
    let a = Trit4(0b10_00_01_00); 
    let b = Trit4(0b10_10_10_00);
    let c = Trit4(0b00_00_00_00);
    let d = Trit4(0b01_01_01_00);
    let code=3;
    
    let result1 =a.gate_core(b,code);
    let result2 = a.gate_core(c,code);
    let result3 = a.gate_core(d,code);



    println!("结果  : {:08b}", result1.0);
    println!("结果  : {:08b}", result2.0);
    println!("结果  : {:08b}", result3.0);
}