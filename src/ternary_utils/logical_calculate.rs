pub mod logical_table;

/// 半加器：返回 (sum, carry)
pub fn ternary_half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum = tsum_gate(a,b);// 和
    let carry=tcons_gate(a, b);// 进位;
    (sum, carry)
}
/// 全加器：基于三维数组实现
pub fn ternary_full_adder(a: u8, b: u8, c_in: u8) -> (u8, u8) {
    let sum =tfullsum_gate(a,b,c_in);// 和
    let carry=tfullcons_gate(a, b, c_in);// 进位
    (sum, carry)
}
///多位三进制加法器,输入两个的三进制向量，返回加法结果向量
pub fn ternary_stack_adder(mut stack1: Vec<u8>,mut stack2: Vec<u8>)-> Vec<u8>{
    let mut result:Vec<u8> = Vec::new();//存储和
    let mut c_in:u8=0;
    
    //Rust标准库中Vec,天然支持后进先出（LIFO），用栈协同弹出，倒序遍历, 支持不同长度
    while !stack1.is_empty() || !stack2.is_empty() {
        let v1 = stack1.pop().unwrap_or(0);
        let v2 = stack2.pop().unwrap_or(0);
 
        let (s_out, next_carry) =ternary_full_adder(v1, v2, c_in);
        result.push(s_out);//存结果
        c_in=next_carry;//进位传递
    }
    result.push(c_in);// 推入最高位
    result.reverse(); // 反转，从高位到低位排列
    result
}

///多位三进制乘法器
pub fn ternary_mul_base(stack1: Vec<u8>, stack2: Vec<u8>)-> Vec<u8>{
    let partial_t: Vec<u8> = stack1.iter().map(|&x| tneg_gate(x)).collect();
    // 构建偏积表：分别是乘以 0, 1, T 的情况
    let partials = vec![
        vec![0; stack1.len()], //0乘任何数，都得0
        stack1.clone(),        //任何数乘1，等于它本身
        partial_t,             //任何数乘T(-1)等于相反数
    ];
    let mut result: Vec<u8> = vec![0];
    for (shift, &m2) in stack2.iter().rev().enumerate() {
        let mut part = partials[m2 as usize].clone();//用偏积表，m2当成下标,出可变副本
        part.resize(part.len() + shift, 0); // 更高效的偏移，低位补 0
        result = ternary_stack_adder(result, part);//加入当前部分积
    }
    result
}



fn tneg_gate(a: u8) -> u8{
    logical_table::TNEG[a as usize]
}

fn tor_gate(a: u8, b: u8) -> u8{
    logical_table::TOR[a as usize][b as usize]
}
fn tand_gate(a: u8, b: u8) -> u8{
    logical_table::TAND[a as usize][b as usize]
}
fn tnor_gate(a: u8, b: u8) -> u8{
    logical_table::TNOR[a as usize][b as usize]
}
fn tnand_gate(a: u8, b: u8) -> u8{
    logical_table::TNAND[a as usize][b as usize]
}
fn txor(a: u8, b: u8) -> u8{
    logical_table::TXOR[a as usize][b as usize]
}
fn txnor(a: u8, b: u8) -> u8{
    logical_table::TXNOR[a as usize][b as usize]
}
fn tsum_gate(a: u8, b: u8) -> u8{
    logical_table::TSUM[a as usize][b as usize]
}
fn tcons_gate(a: u8, b: u8) -> u8{
    logical_table::TCONS[a as usize][b as usize]
}
fn tany_gate(a: u8, b: u8) -> u8{
    logical_table::TANY[a as usize][b as usize]
}


fn tfullsum_gate(a: u8, b: u8, c: u8) -> u8{
    logical_table::TFULLSUM[a as usize][b as usize][c as usize]
}
fn tfullcons_gate(a: u8, b: u8, c: u8) -> u8{
    logical_table::TFULLCONS[a as usize][b as usize][c as usize]
}

