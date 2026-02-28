use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {

    let a = trits!("010T");
    // let b = trits!("---0");
    // let c = trits!("0000");
    // let d = trits!("+++0");

    let result1=a.to_sign();


    println!("{}",result1);
    // println!("{}",result2);
    // println!("{}",result3);
}




















// use ternary_arithmetic::ternary_asm::asm_utils;

// pub static PROGRAM: &[&str] = &[
//     "0000_0000_00T0",// LOAD REG0,T010
//     "0100_00TT_000T",// COPY REG0,REGS
//     // "0000_0000_T010",// LOAD REG0,TTT0

//     // "1T00_00TT_00TT",// CALC REG0,REGS,code

//     "1000_0000_00T0",
// ];

// fn main() -> std::io::Result<()> {
//     asm_utils::write_tasm(PROGRAM)?;
//     asm_utils::write_tbin()?;    
//     asm_utils::read_tbin()?;     
//     asm_utils::run_from_tbin()?;
//     Ok(())
// }

// use trit_macro::trits;
// use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;
// fn main() {
//     let a = trits!("T010");
//     let b = trits!("---0");
//     let c = trits!("0000");
//     let d = trits!("+++0");

//     let code=9;
    
//     let result1 =a.gate_core(b,code);
//     let result2 =a.gate_core(c,code);
//     let result3 =a.gate_core(d,code);


//     println!("{}",result1);
//     println!("{}",result2);
//     println!("{}",result3);
// }