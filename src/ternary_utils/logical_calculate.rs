pub mod logical_table;

/// 半加器：返回 (sum, carry)
pub fn ternary_half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum = tsum_gate(a,b);// 和
    let carry=tcons_gate(a, b);// 进位;
    (sum, carry)
}
/// 全加器2：基于半加器实现
pub fn ternary_full_adder2(a: u8, b: u8, c_in: u8) -> (u8, u8) {
    //2个平衡三进制半加器及1个平衡三进制调和门,组成一个平衡三进制全加器
    let (num,c1_in)=ternary_half_adder(a,b);
    let (sum,c2_in)=ternary_half_adder(num,c_in);
    let carry=tany_gate(c1_in, c2_in);//两个进位数合成一个进位数;
    (sum, carry)
}
/// 全加器：基于三维数组实现
pub fn ternary_full_adder(a: u8, b: u8, c_in: u8) -> (u8, u8) {
    let sum =tfullsum_gate(a,b,c_in);// 和
    let carry=tfullcons_gate(a, b, c_in);// 进位
    (sum, carry)
}


///多位三进制加法器基础,输入两个的三进制向量，返回加法结果向量和最终进位
pub fn ternary_stackadder_base(mut stack1: Vec<u8>,mut stack2: Vec<u8>,carry_in: u8)-> (Vec<u8>, u8){
    let mut result:Vec<u8> = Vec::new();//存储和
    let mut c_in:u8=carry_in;
    
    //Rust标准库中Vec,天然支持后进先出（LIFO），用栈协同弹出，倒序遍历, 支持不同长度
    while !stack1.is_empty() || !stack2.is_empty() {
        let v1 = stack1.pop().unwrap_or(0);
        let v2 = stack2.pop().unwrap_or(0);

        let (s_out, next_carry) =ternary_full_adder(v1, v2, c_in);
        result.push(s_out);//存结果
        c_in=next_carry;//进位传递
    }
    result.reverse(); // 反转，从高位到低位排列
    (result, c_in)
}

//多位三进制加法器
pub fn ternary_stack_adder(stack1: Vec<u8>,stack2: Vec<u8>) -> Vec<u8> {
    let (mut result, carry) = ternary_stackadder_base(stack1,stack2, 0);
    result.insert(0, carry);
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

