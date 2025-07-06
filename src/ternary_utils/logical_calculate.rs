pub mod logical_table;

/// 获取相反数
pub fn ternary_tneg(stack: Vec<u8>)-> Vec<u8>{
    stack.iter().map(|&x| logical_table::TNEG[x as usize]).collect()
}
/// 判断三进制数的符号（从高位到低位）
/// - 返回 0 表示全 0
/// - 返回 1 表示正数（首个非零位是 1）
/// - 返回 2 表示负数（首个非零位是 T）
pub fn ternary_sign(stack: &[u8]) -> u8 {
    let mut state: u8 = 0;
    for &digit in stack {
        state = logical_table::TPOZ[state as usize][digit as usize];
    }
    state
}

/// 半加器：返回 (sum, carry)
pub fn ternary_half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum = logical_table::TSUM[a as usize][b as usize];// 和
    let carry=logical_table::TCONS[a as usize][b as usize];// 进位;
    (sum, carry)
}
/// 全加器：基于三维数组实现
pub fn ternary_full_adder(a: u8, b: u8, c_in: u8) -> (u8, u8) {
    let sum =logical_table::TFULLSUM[a as usize][b as usize][c_in as usize];// 和
    let carry=logical_table::TFULLCONS[a as usize][b as usize][c_in as usize];// 进位
    (sum, carry)
}
/// 多位三进制累加器
pub fn ternary_stack_accumulate(mut stack: Vec<u8>,mut c_in: u8)-> Vec<u8>{
    let mut result:Vec<u8> = Vec::new();//存储和
    while let Some(v1) = stack.pop() {
        let (s_out, next_carry) = ternary_half_adder(v1, c_in);
        result.push(s_out);//存结果
        c_in = next_carry;//进位传递
    }
    if c_in!=0{
        result.push(c_in);// 推入最高位
    }
    result.reverse(); // 反转，从高位到低位排列
    result
}
/// 多位三进制加法器,输入两个的三进制向量，返回加法结果向量和最终进位
pub fn ternary_stack_adder(mut stack1: Vec<u8>,mut stack2: Vec<u8>)-> Vec<u8>{
    let mut result:Vec<u8> = Vec::new();//存储和
    let mut c_in:u8=0;
    
    //Rust标准库中Vec，用栈协同弹出，倒序遍历, 支持不同长度
    while !stack1.is_empty() || !stack2.is_empty() {
        let v1 = stack1.pop().unwrap_or(0);
        let v2 = stack2.pop().unwrap_or(0);
 
        let (s_out, next_carry) =ternary_full_adder(v1, v2, c_in);
        result.push(s_out);//存结果
        c_in=next_carry;//进位传递
    }
    if c_in!=0{
        result.push(c_in);// 推入最高位
    }
    result.reverse(); // 反转，从高位到低位排列
    result
}

/// 多位三进制乘法器
pub fn ternary_mul_base(stack1: Vec<u8>, stack2: Vec<u8>)-> Vec<u8>{
    // 构建偏积表：分别是乘以 0, 1, T 的情况
    let partials = vec![
        vec![0; stack1.len()], //0乘任何数，都得0
        stack1.clone(),        //任何数乘1，等于它本身
        ternary_tneg(stack1),  //任何数乘T(-1)等于相反数
    ];
    let mut result: Vec<u8> = vec![0];
    for (shift, &m2) in stack2.iter().rev().enumerate() {
        let mut part = partials[m2 as usize].clone();//用偏积表，m2当成下标,出可变副本
        part.resize(part.len() + shift, 0); // 更高效的偏移，低位补 0
        result = ternary_stack_adder(result, part);//加入当前部分积
    }
    result
}

///多位三进制除法器：累加版
pub fn ternary_div_base(stack1: Vec<u8>, stack2: Vec<u8>)-> (Vec<u8>,Vec<u8>){
    let sign_divided=ternary_sign(&stack1);//最初被除数符号
    let sign_divisor=ternary_sign(&stack2);//除数符号

    // 零不能作为除数
    if sign_divisor == 0 {
        panic!("零不能作为除数");
    }
    //零除于任何非零数都等于零
    if sign_divided == 0{
        return (vec![0], vec![0]);
    }

    let is_equal=sign_divided==sign_divisor;
    //被除数与除数，要互为相反数，先将被除数取反，一直加直到加回来 
    let mut remainder = if is_equal{ternary_tneg(stack1)}else{stack1};
    let digit = if is_equal { 1 } else { 2 }; //同符号上1，反之上T
    let mut count = 0;//统计加了多少次

    loop{
        let current_sign=ternary_sign(&remainder);//获取当前被除数符号
        if current_sign==sign_divisor{//如果加回来，被除数与除数符号相同
            //撤回上一步结果
            count -= 1;
            if is_equal{
                remainder=ternary_stack_adder(ternary_tneg(remainder), stack2.clone());
            }else {
                remainder=ternary_stack_adder(ternary_tneg(stack2.clone()), remainder);
            }
            break;
        }
        count += 1;
        remainder = ternary_stack_adder(remainder, stack2.clone());//取反的被除数不断加除数
    }

    //构建最终商
    let mut quotient = vec![0];
    for _ in 0..count {
        quotient = ternary_stack_accumulate(quotient, digit);
    }
 
    //商、余数
    (quotient,remainder)
}



