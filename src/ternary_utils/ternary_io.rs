// 导入 ternary_logic 模块
use crate::ternary_utils::logical_calculate;

// **全局三进制字符映射**
const BALANCE_INDEX:[char; 3]=['0', '+', '-'];

/// trit 向量转换为字符串,输入时检测过，输出则不检测
fn decode_trits(trits: &[u8])->String {
    trits.iter().map(|&t| BALANCE_INDEX[t as usize]).collect()
}

/// 字符串编码成 trit 向量，('0','+','-') 映射为 trit 数值 (0,1,2)
fn encode_trits(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            '0' => 0,
            '+' => 1,
            '-' => 2,
            other => panic!("Invalid character in trit string: '{}'", other),
        })
        .collect()
}

//平衡三进制栈多位加器输出
pub fn test_stack_adder(s1: &str ,s2:&str){
    let stack1: Vec<u8> = encode_trits(&s1);
    let stack2: Vec<u8> = encode_trits(&s2);

    let sum =logical_calculate::ternary_stack_adder(stack1, stack2);
    println!("结果: {}", decode_trits(&sum));
}

//平衡三进制全加器输出
pub fn test_full_adder(a: u8, b: u8, c_in: u8){
    let (sum, carry) =logical_calculate::ternary_full_adder(a, b, c_in);
    println!(
        "输入: a={} b={} c_in={} => 输出:sum={}, carry={}",
        a, b, c_in, sum, carry
    );
}
//平衡三进制半加器输出
pub fn test_half_adder(a: u8, b: u8){
    let (sum, carry) =logical_calculate::ternary_half_adder(a, b);
    println!(
        "输入: a={} b={} => 输出:sum={}, carry={}",
        a, b, sum, carry
    );
}

