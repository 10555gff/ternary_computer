 use ternary_arithmetic::ternary_cpu::trit::Trit4;

fn main() {
    let reg_a = Trit4(0b01_00_10_01); // 对应三进制序列 [P, Z, N, P]
    let reg_b = Trit4(0b00_01_01_10); // 对应三进制序列 [Z, P, P, N]

    // 执行并行或运算
    let result = reg_a.dibit_tor(reg_b);
    

    println!("结果内部值: {:08b}", result);
}
