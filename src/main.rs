use ternary_arithmetic::logical_calculate::Digit;




// fn bit_gate(a: u8, b: u8) -> u8 {
//     // 针对一个 2-bit 单元的逻辑
//     if a == 0b10 || b == 0b10 {
//         0b10
//     } else if a == 0b01 && b == 0b01 {
//         0b01
//     } else {
//         0b00
//     }
// }

// /// 对整个字节进行 4 组双 bit 逻辑
// fn dibit_gate(a: u8, b: u8) -> u8 {
//     let mut result = 0u8;
//     for i in 0..4 {
//         let aa = (a >> (i * 2)) & 0b11; // 提取 a 的第 i 个双 bit
//         let bb = (b >> (i * 2)) & 0b11; // 提取 b 的第 i 个双 bit
//     println!("aa = {:02b}", aa);

//         let rr = bit_gate(aa, bb);      // 应用双 bit 门逻辑
//         result |= rr << (i * 2);        // 写回结果
//     }
//     result
// }

fn main() {
    let a = 0b10_00_01_00;

    let b = 0b10_10_10_00;
    let c = 0b00_00_00_00;
    let d = 0b01_01_01_00;

    let r =Digit:: dibit_gate(a, c);

    println!("a = {:08b}", a);
    println!("b = {:08b}", b);
    println!("r = {:08b}", r);
}
