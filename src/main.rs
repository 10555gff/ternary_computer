// use trit_macro::trits;
// use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

// fn main() {
//     let a = trits!("1110");
//     let b = trits!("1110");
//     let result1=a>=b;
//     println!("{}",result1);
// }






use ternary_arithmetic::ternary_asm::asm_utils;

pub static PROGRAM: &[&str] = &[
    "0000_0000_0TTT",// LOAD REG0
    "0100_00TT_0011",// COPY REG0,REG8

    "0000_0000_0001",// LOAD REG0
    "0100_00TT_0010",// COPY REG0,REG7

    "1T00_0011_001T",// CALC A*REG8,REG7,ADD

    "1000_0000_000T",//










    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD

    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD

//    






    // "0000_0000_0001",// LOAD REG0
    // "0100_00TT_0010",// COPY REG0,REG7

    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD
    // "1T00_0011_001T",// CALC A*REG8,REG7,ADD

];

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM)?;
    asm_utils::write_tbin()?;    
    asm_utils::read_tbin()?;     
    asm_utils::run_from_tbin()?;
    Ok(())
}

// use trit_macro::trits;
// use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;
// fn main() {
//     let a = trits!("T010");
//     let b = trits!("---0");
//     let c = trits!("0000");
//     let d = trits!("+++0");

//     let code=0;
    
//     let result1 =a.gate_core(b,code);
//     let result2 =a.gate_core(c,code);
//     let result3 =a.gate_core(d,code);


//     println!("{}",result1);
//     println!("{}",result2);
//     println!("{}",result3);
// }