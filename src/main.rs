const OFFSET: u8 = 48;

const DSUM: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
    [2, 3, 4, 5, 6, 7, 8, 9, 0, 1],
    [3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
    [4, 5, 6, 7, 8, 9, 0, 1, 2, 3],
    [5, 6, 7, 8, 9, 0, 1, 2, 3, 4],
    [6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
    [7, 8, 9, 0, 1, 2, 3, 4, 5, 6],
    [8, 9, 0, 1, 2, 3, 4, 5, 6, 7],
    [9, 0, 1, 2, 3, 4, 5, 6, 7, 8],
];

const DCOMS: [[u8; 10]; 10] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const DSUB: [[u8; 10]; 10] = [
    [0, 9, 8, 7, 6, 5, 4, 3, 2, 1],
    [1, 0, 9, 8, 7, 6, 5, 4, 3, 2],
    [2, 1, 0, 9, 8, 7, 6, 5, 4, 3],
    [3, 2, 1, 0, 9, 8, 7, 6, 5, 4],
    [4, 3, 2, 1, 0, 9, 8, 7, 6, 5],
    [5, 4, 3, 2, 1, 0, 9, 8, 7, 6],
    [6, 5, 4, 3, 2, 1, 0, 9, 8, 7],
    [7, 6, 5, 4, 3, 2, 1, 0, 9, 8],
    [8, 7, 6, 5, 4, 3, 2, 1, 0, 9],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];

const DBORROW: [[u8; 10]; 10] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const DMULSUM: [[u8; 10]; 10] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [0, 2, 4, 6, 8, 0, 2, 4, 6, 8],
    [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
    [0, 4, 8, 2, 6, 0, 4, 8, 2, 6],
    [0, 5, 0, 5, 0, 5, 0, 5, 0, 5],
    [0, 6, 2, 8, 4, 0, 6, 2, 8, 4],
    [0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
    [0, 8, 6, 4, 2, 0, 8, 6, 4, 2],
    [0, 9, 8, 7, 6, 5, 4, 3, 2, 1],
];

const DMULCOMS: [[u8; 10]; 10] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 2, 2, 2, 3, 3],
    [0, 0, 1, 1, 2, 2, 3, 3, 4, 4],
    [0, 0, 1, 1, 2, 3, 3, 4, 4, 5],
    [0, 0, 1, 2, 2, 3, 4, 4, 5, 6],
    [0, 0, 1, 2, 3, 4, 4, 5, 6, 7],
    [0, 0, 1, 2, 3, 4, 5, 6, 7, 8],
];

//大数比较
fn decimal_cmp(num1: &str, num2: &str) -> u8 {
    match (num1.len().cmp(&num2.len())).then(num1.cmp(num2)) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => 2,
        std::cmp::Ordering::Equal => 0,
    }
}
//无符号减法
fn subtract_unsigned(str1: &str, str2: &str) -> String {
    let max_length = std::cmp::max(str1.len(), str2.len());
    let str1 = format!("{:0>width$}", str1, width = max_length);
    let str2 = format!("{:0>width$}", str2, width = max_length);
    let mut result = vec!['0'; max_length];
    let mut pre_borrow = 0;
    let mut now_borrow;
    let mut pre_diff;
    let mut now_diff;

    //println!("{}\n-\n{}\n=", str1, str2);

    for i in (0..max_length).rev() {
        let a = (str1.as_bytes()[i] - OFFSET) as usize;
        let b = (str2.as_bytes()[i] - OFFSET) as usize;
        pre_diff = DSUB[a][b];
        now_diff = DSUB[pre_diff as usize][pre_borrow as usize];
        now_borrow = if DBORROW[a][b] != 0 || DBORROW[pre_diff as usize][pre_borrow as usize] != 0 {
            1
        } else {
            0
        };
        pre_borrow = now_borrow;
        result[i] = (OFFSET + now_diff as u8) as char;
    }

    let mut result = result.into_iter().collect::<String>();
    if pre_borrow != 0 {
        panic!("Result is negative, not supported in this implementation");
    }
    // 去除前导零
    result = result.trim_start_matches('0').to_string();
    if result.is_empty() {
        result = "0".to_string();
    }
    result
}

// 逐位减法
fn decimal_subtractor(str1: &str, str2: &str) -> String {
    match decimal_cmp(str1, str2) {
        0 => return "0".to_string(), // 相等
        1 => subtract_unsigned(str1, str2), //保证str1 >= str2，直接减
        2 =>format!("-{}", subtract_unsigned(str2, str1)),// str1 < str2，交换后加负号
        _ => unreachable!(),
    }
}

// 逐位加法
fn decimal_adder(str1: &str, str2: &str) -> String {
    let max_length = std::cmp::max(str1.len(), str2.len());
    let str1 = format!("{:0>width$}", str1, width = max_length);
    let str2 = format!("{:0>width$}", str2, width = max_length);
    let mut result = vec!['0'; max_length];
    let mut pre_carry = 0;
    let mut now_carry;
    let mut pre_sum;
    let mut now_sum;

    for i in (0..max_length).rev() {
        let a = (str1.as_bytes()[i] - OFFSET) as usize;
        let b = (str2.as_bytes()[i] - OFFSET) as usize;
        pre_sum = DSUM[a][b];
        now_sum = DSUM[pre_carry as usize][pre_sum as usize];
        now_carry = if DCOMS[a][b] != 0 || DCOMS[pre_carry as usize][pre_sum as usize] != 0 {
            1
        } else {
            0
        };
        pre_carry = now_carry;
        result[i] = (OFFSET + now_sum as u8) as char;
    }

    let mut result = result.into_iter().collect::<String>();
    if pre_carry != 0 {
        result = format!("1{}", result);
    }
    result
}

// 执行单位的乘法
fn multiply_step(str: &str, c: char, max_length: usize) -> String {
    let mut sum_result = vec!['0'; max_length + 1];
    let mut carry_result = vec!['0'; max_length + 1];
    let b = (c as u8 - OFFSET) as usize;

    for (i, &char) in str.as_bytes().iter().enumerate() {
        let a = (char - OFFSET) as usize;
        sum_result[i + 1] = (OFFSET + DMULSUM[a][b] as u8) as char;
        carry_result[i] = (OFFSET + DMULCOMS[a][b] as u8) as char;
    }

    decimal_adder(&sum_result.into_iter().collect::<String>(), &carry_result.into_iter().collect::<String>())
}

// 逐位乘法
fn decimal_multiply(str1: &str, str2: &str) -> String {
    let (max_string, min_string) = if str1.len() > str2.len() {
        (str1, str2)
    } else {
        (str2, str1)
    };
    let min_length = min_string.len();
    let mut string_array = Vec::new();
    let mut return_result = String::new();

    for i in 0..min_length {
        return_result = multiply_step(max_string, min_string.chars().nth(min_length - 1 - i).unwrap(), max_string.len());
        return_result = format!("{}{}", return_result, "0".repeat(i));
        string_array.push(return_result.clone());
    }

    for i in 0..string_array.len() - 1 {
        return_result = decimal_adder(&string_array[i], &return_result);
    }

    if !return_result.is_empty() && return_result.starts_with('0') {
        return_result = return_result.trim_start_matches('0').to_string();
    }
    if return_result.is_empty() {
        return_result = "0".to_string();
    }

    return_result
}




fn main() {
    let s1 = "1234891588";
    let s2 = "1234891588";

    let sub = decimal_subtractor(s1, s2);
    println!("{s1}-{s2}={sub}");

    let add = decimal_adder(s1, s2);
    println!("{s1}+{s2}={add}");

    let mul = decimal_multiply(s1, s2);
    println!("{s1}*{s2}={mul}");
}


