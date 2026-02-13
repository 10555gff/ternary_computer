use ternary_arithmetic::ternary_cpu::trit::Trit4;

// fn main() {
//     let reg_a = Trit4(0b01_00_10_01); // 对应三进制序列 [P, Z, N, P]
//     let reg_b = Trit4(0b00_01_01_10); // 对应三进制序列 [Z, P, P, N]

//     // 执行并行或运算
//     let result = reg_a.dibit_txor(reg_b);
    

//     println!("结果内部值: {:08b}", result.0);
// }




fn main() {
    //let a = Trit4(0b10_00_01_00); 
    // let b = Trit4(0b10_10_10_00);
    // let c = Trit4(0b00_00_00_00);
    // let d = Trit4(0b01_01_01_00);

    let a = Trit4(0b01_01_10_01); 
    
    let result1 = a.tneg();
    // let result2 = a.tor(c);
    // let result3 = a.tor(d);



    println!("结果  : {:08b}", result1.0);
    // println!("结果  : {:08b}", result2.0);
    // println!("结果  : {:08b}", result3.0);
}