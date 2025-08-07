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

fn subtract_unsigned(bytes1: &[u8], bytes2: &[u8]) -> String {
    let mut result =  Vec::new();
    let (mut i,mut j)=(bytes1.len(),bytes2.len());

    let mut pre_borrow = 0;

    while i > 0 || j > 0 {
        let mut a = 0;
        if i > 0 {
            i -= 1;
            a = bytes1[i] - OFFSET;
        }
        let mut b = 0;
        if j > 0 {
            j -= 1;
            b = bytes2[j] - OFFSET;
        }

        let pre_diff = DSUB[a as usize][b as usize];
        let now_diff = DSUB[pre_diff as usize][pre_borrow as usize];
        pre_borrow =if DBORROW[a as usize][b as usize] != 0 || DBORROW[pre_diff as usize][pre_borrow as usize] != 0 {1} else {0};
        result.push((OFFSET + now_diff) as char);
        //println!("{}:{}", a, b);
    }
    result.reverse();
    result.into_iter().collect()
}
// 逐位减法
pub fn decimal_subtractor(str1: &str, str2: &str) -> String {
    match decimal_cmp(str1, str2) {
        0 => return "0".to_string(), // 相等
        1 => subtract_unsigned(str1.as_bytes(), str2.as_bytes()), //保证str1 >= str2，直接减
        2 =>format!("-{}", subtract_unsigned(str2.as_bytes(), str1.as_bytes())),// str1 < str2，交换后加负号
        _ => unreachable!(),
    }
}

// 逐位加法
pub fn decimal_adder(num1: &str, num2: &str) -> String {
    let bytes1 = num1.as_bytes();
    let bytes2 = num2.as_bytes();
    let mut result =  Vec::new();

    let (mut i,mut j)=(num1.len(),num2.len());

    let mut pre_carry = 0;

    while i > 0 || j > 0 {
        let mut a = 0;
        if i > 0 {
            i -= 1;
            a = bytes1[i] - OFFSET;
        }
        let mut b = 0;
        if j > 0 {
            j -= 1;
            b = bytes2[j] - OFFSET;
        }

        let pre_sum = DSUM[a as usize][b as usize];
        let now_sum = DSUM[pre_carry as usize][pre_sum as usize];
        pre_carry = if DCOMS[a as usize][b as usize] != 0 || DCOMS[pre_carry as usize][pre_sum as usize] != 0 {1} else {0};
        result.push((OFFSET + now_sum) as char);
       // println!("{}:{}", a, b);
    }
    if pre_carry != 0 {
        result.push('1');
    }

    result.reverse();
    result.into_iter().collect()
}


// 执行单位的乘法
fn multiply_step(str: &[u8], c: u8) -> String {
    let mut sum_result = String::new();
    let mut carry_result = String::new();

    let a = c - OFFSET;
    for &byte in str{
        let b = byte - OFFSET;
        let sum= (OFFSET + DMULSUM[a as usize][b as usize]) as char;
        let carry= (OFFSET + DMULCOMS[a as usize][b as usize]) as char;

        sum_result.push(sum);
        carry_result.push( carry);
    }
    carry_result.push('0');
    decimal_adder(&carry_result,&sum_result)
}


// 逐位乘法
pub fn decimal_multiply(num1: &str, num2: &str) -> String {
    let (max_string, min_string) = if num1.len() > num2.len() {(num1.as_bytes(), num2.as_bytes())} else {(num2.as_bytes(), num1.as_bytes())};

    let min_length = min_string.len()-1;
    let mut string_array = String::new();

    for (shift,&byte) in min_string.iter().enumerate() {
        let zeros = "0".repeat(min_length-shift);
        let return_result = format!("{}{}", multiply_step(max_string, byte), zeros);

        println!("{:?}:{:?}",return_result,shift);
        string_array=decimal_adder(&return_result,&string_array);//所以结果累加到一起
    }
    string_array
}

