use ternary_arithmetic::ternary_cpu::trit::Trit4;

// fn main() {
//     let reg_a = Trit4(0b01_00_10_01); // 对应三进制序列 [P, Z, N, P]
//     let reg_b = Trit4(0b00_01_01_10); // 对应三进制序列 [Z, P, P, N]

//     // 执行并行或运算
//     let result = reg_a.dibit_txor(reg_b);
    

//     println!("结果内部值: {:08b}", result.0);
// }




fn main() {
    let input_a = Trit4(0b1010_1010); 
    let input_b = Trit4(0b0111_0101);
    
    let result = input_a.tor(input_b);
    //Trit4::custom_logic_process(input_a, input_b);

    println!("输入 A: {:08b}", input_a.0);
    println!("输入 B: {:08b}", input_b.0);
    println!("结果  : {:08b}", result.0);
}