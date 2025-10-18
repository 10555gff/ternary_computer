use core::ops::{Neg, Not, BitOr, BitAnd, BitXor, Add, Sub, Mul, Div};
use std::cmp::{Ordering, PartialOrd};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP,TDIV,T3OR,T3AND,TFULLSUM,TFULLCONS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    Z =0,
    P =1,
    N =2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitResult {
    pub carry: Digit,
    pub sum: Digit,
}

pub mod logical_table;

impl Digit {
pub const fn to_char(&self) -> char {
    match self {
        Digit::Z=> '0',
        Digit::P => '+',
        Digit::N => '-',
    }
}
pub const fn from_char(c: char) -> Digit {
    match c {
        '0' => Digit::Z,
        '+' => Digit::P,
        '-' =>  Digit::N,
        _ =>  panic!("Invalid value. A Ternary must be either -, 0 or +."),
    }
}

pub const fn to_char_t(&self) -> char {
    match self {
        Digit::Z=>'0',
        Digit::P=>'1',
        Digit::N=>'T',
    }
}
pub const fn from_char_t(c: char) -> Digit {
    match c {
        '0' => Digit::Z,
        '1' => Digit::P,
        'T' => Digit::N,
        _ => panic!("Invalid value. Expected 'T', '0', or '1'."),
    }
}

pub const fn to_u8(&self) -> u8 {
    match self {
        Digit::Z=> 0,
        Digit::P=> 1,
        Digit::N=> 2,
    }
}
pub const fn from_u8(i: u8) -> Digit {
    match i {
        0 => Digit::Z,
        1 => Digit::P,
        2 => Digit::N,
        _ => panic!("Invalid value. A Ternary must be either -1, 0 or +1."),
    }
}

pub const fn to_i8(&self) -> i8 {
    match self {
        Digit::Z=> 0,
        Digit::P=> 1,
        Digit::N=>-1,
    }
}
pub const fn from_i8(i: i8) -> Digit {
    match i {
        0 => Digit::Z,
        1 => Digit::P,
       -1 => Digit::N,
        _ => panic!("Invalid value. A Ternary must be either -1, 0 or +1."),
    }
}

pub const fn to_neg(self) -> Self {
    match self {
        Digit::Z => Digit::Z,
        Digit::P => Digit::N,
        Digit::N => Digit::P,
    }
}

/// Determines the condition of RIGHT_MUX for the current `Digit`.
pub const fn post(self) -> Self {
    match self {
        Digit::Z => Digit::P,
        Digit::P => Digit::N,
        Digit::N => Digit::Z,
    }
}

/// Determines the condition of LEFT_MUX for the current `Digit`.
pub const fn pre(self) -> Self {
    match self {
        Digit::Z => Digit::N,
        Digit::P => Digit::Z,
        Digit::N => Digit::P,
    }
}

/// Determines the condition of MAX_MUX for the current `Digit`.
pub const fn max(self) -> Self {
    match self {
        Digit::Z => Digit::Z,
        Digit::P => Digit::P,
        Digit::N => Digit::Z,
    }
}

/// Determines the condition of MIN_MUX for the current `Digit`.
pub const fn min(self) -> Self {
    match self {
        Digit::Z => Digit::Z,
        Digit::P => Digit::Z,
        Digit::N => Digit::N,
    }
}

pub const fn tor(self, other: Self) -> Self {
    TOR[self as usize][other as usize]
}
pub const fn tand(self, other: Self) -> Self {
    TAND[self as usize][other as usize]
}
pub const fn tnor(self, other: Self) -> Self {
    TNOR[self as usize][other as usize]
}
pub const fn tnand(self, other: Self) -> Self {
    TNAND[self as usize][other as usize]
}
pub const fn txor(self, other: Self) -> Self {
    TXOR[self as usize][other as usize]
}
pub const fn txnor(self, other: Self) -> Self {
    TXNOR[self as usize][other as usize]
}
pub const fn tsum(self, other: Self) -> Self {
    TSUM[self as usize][other as usize]
}
pub const fn tcons(self, other: Self) -> Self {
    TCONS[self as usize][other as usize]
}
pub const fn tany(self, other: Self) -> Self {
    TANY[self as usize][other as usize]
}
pub const fn tpoz(self, other: Self) -> Self {
    TPOZ[self as usize][other as usize]
}
pub const fn tcmp(self, other: Self) -> Self {
    TCMP[self as usize][other as usize]
}
pub const fn tdiv(self, other: Self) -> Option<Self> {
    TDIV[self as usize][other as usize]
}
pub const fn t3or(self, b: Self,c: Self) -> Self {
    T3OR[self as usize][b as usize][c as usize]
}
pub const fn t3and(self, b: Self,c: Self) -> Self {
    T3AND[self as usize][b as usize][c as usize]
}
pub const fn half_adder(self, other: Self) -> DigitResult {
    DigitResult {
        carry: TCONS[self as usize][other as usize],
        sum: TSUM[self as usize][other as usize],
    }
}
pub const fn full_adder(self, b: Self, c_in: Self) -> DigitResult {
    DigitResult {
        carry: TFULLCONS[self as usize][b as usize][c_in as usize],
        sum: TFULLSUM[self as usize][b as usize][c_in as usize],
    }
}

//2个半加器及1个调和门,组成一个平衡三进制全加器
pub fn full_adder_gate(self, b: Self, c_in: Self) -> DigitResult {
    let half_adder1=Digit::half_adder(self, b);
    let mut half_adder2=Digit::half_adder(half_adder1.sum, c_in);
    let full_carry=TANY[half_adder1.carry as usize][half_adder2.carry as usize];//两个进位数合成一个进位数;
    half_adder2.carry=full_carry;
    half_adder2
}

}


pub trait DibitLogic {
    fn digits_print(&self);
    fn digits_print_t(&self);

    fn dibit_gate_table(&self, other: u8, table: &[[Digit; 3]; 3]) -> u8;
    fn dibit_tor(&self, other: u8) -> u8;
    fn dibit_tand(&self, other: u8) -> u8;
    fn dibit_tnor(&self, other: u8) -> u8;
    fn dibit_tnand(&self, other: u8) -> u8;
    fn dibit_txor(&self, other: u8) -> u8;
    fn dibit_txnor(&self, other: u8) -> u8;
    fn dibit_tsum(&self, other: u8) -> u8;
    fn dibit_tcons(&self, other: u8) -> u8;
    fn dibit_tany(&self, other: u8) -> u8;
    fn dibit_tpoz(&self, other: u8) -> u8;
    fn dibit_tcmp(&self, other: u8) -> u8;

    fn dibit_gate_table3(&self, b: Self, c_in: Self, table: &[[[Digit; 3]; 3]; 3]) -> u8;
    fn dibit_t3or(&self, b: u8, c_in: u8) -> u8;
    fn dibit_t3and(&self, b: u8, c_in: u8) -> u8;
    fn dibit_tfullsum(&self, b: u8, c_in: u8) -> u8;
    fn dibit_tfullcons(&self, b: u8, c_in: u8) -> u8;
    fn dibit_gate_full(&self, other: &Self, c_in: Digit) -> (u8,u8);

    
}

impl DibitLogic for u8 {
    fn digits_print(&self) {
        for i in (0..4).rev() {
            let d = Digit::from_u8((self >> (i * 2)) & 0b11);// 提取 a 的第 i 个双 bit
            print!("{}",d.to_char());
        }
        println!();
    }
    fn digits_print_t(&self) {
        for i in (0..4).rev() {
            let d = Digit::from_u8((self >> (i * 2)) & 0b11);// 提取 a 的第 i 个双 bit
            print!("{}",d.to_char_t());
        }
        println!();
    }

    /// 对整个字节进行双 bit 逻辑门，支持不同的逻辑操作
    fn dibit_gate_table(&self, other: u8, table: &[[Digit; 3]; 3]) -> u8 {
        // 直接使用逻辑表索引，避免闭包开销
        let r0 = table[(self & 0b11) as usize][(other & 0b11) as usize] as u8;
        let r1 = table[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize] as u8;
        
        r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
    }

    /// 使用函数别名，便于调用
    fn dibit_tor(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TOR)
    }
    fn dibit_tand(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TAND)
    }
    fn dibit_tnor(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TNOR)
    }
    fn dibit_tnand(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TNAND)
    }
    fn dibit_txor(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TXOR)
    }
    fn dibit_txnor(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TXNOR)
    }
    fn dibit_tsum(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TSUM)
    }
    fn dibit_tcons(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TCONS)
    }
    fn dibit_tany(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TANY)
    }
    fn dibit_tpoz(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TPOZ)
    }
    fn dibit_tcmp(&self, other: u8) -> u8 {
        self.dibit_gate_table(other, &TCMP)
    }


    fn dibit_gate_table3(&self, b: Self, c_in: Self, table: &[[[Digit; 3]; 3]; 3]) -> u8 {
        // 直接使用逻辑表索引，避免闭包开销
        let r0 = table[(self & 0b11) as usize][(b & 0b11) as usize][(c_in & 0b11) as usize] as u8;
        let r1 = table[((self >> 2) & 0b11) as usize][((b >> 2) & 0b11) as usize][((c_in >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self >> 4) & 0b11) as usize][((b >> 4) & 0b11) as usize][((c_in >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self >> 6) & 0b11) as usize][((b >> 6) & 0b11) as usize][((c_in >> 6) & 0b11) as usize] as u8;
        r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
    }

    fn dibit_t3or(&self, b: u8, c_in: u8) -> u8 {
        self.dibit_gate_table3(b, c_in,&T3OR)
    }
    fn dibit_t3and(&self, b: u8, c_in: u8) -> u8 {
        self.dibit_gate_table3(b, c_in,&T3AND)
    }

    fn dibit_tfullsum(&self, b: u8, c_in: u8) -> u8 {
        self.dibit_gate_table3(b, c_in,&TFULLSUM)
    }
    fn dibit_tfullcons(&self, b: u8, c_in: u8) -> u8 {
        self.dibit_gate_table3(b, c_in,&TFULLCONS)
    }



    fn dibit_gate_full(&self, other: &Self, c_in: Digit) -> (u8,u8) {
         // 4trit全加器
        let sum0 = TFULLSUM[(self & 0b11) as usize][(other & 0b11) as usize][c_in as usize] as u8;// 和
        let carry0 = TFULLCONS[(self & 0b11) as usize][(other & 0b11) as usize][c_in as usize];// 进位


        let sum1 = TFULLSUM[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize][carry0 as usize] as u8;
        let carry1 = TFULLCONS[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize][carry0 as usize];

        let sum2 = TFULLSUM[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize][carry1 as usize] as u8;
        let carry2 = TFULLCONS[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize][carry1 as usize];

        let sum3 = TFULLSUM[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize][carry2 as usize] as u8;
        let carry3 = TFULLCONS[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize][carry2 as usize] as u8;

        
        let sum=sum0 | (sum1 << 2) | (sum2 << 4) | (sum3 << 6);
        (carry3 ,sum)        
    }






}


//Neg -x	数学意义的“负号”运算
impl Neg for Digit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Digit::Z => Digit::Z,
            Digit::P => Digit::N,
            Digit::N => Digit::P,
        }
    }
}
//Not !x	按位/逻辑意义的“非”或“取反”运算
impl Not for Digit {
    type Output = Self;

    fn not(self) -> Self::Output {
        -self  //行为一致，直接重用 Neg 实现
    }
}

/// Performs a bitwise OR (`|`) operation between two `Digit` values and returns the result.
impl BitOr for Digit {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        self.tor(other)
    }
}
/// Performs a bitwise AND (`&`) operation between two `Digit` values and returns the result.
impl BitAnd for Digit {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        self.tand(other)
    }
}

/// Performs a bitwise XOR(`^`) (exclusive OR) operation between two `Digit` values.
impl BitXor for Digit {
    type Output = Self;
    
    fn bitxor(self, other: Self) -> Self::Output {
        self.txor(other)
    }
}

/// Multiplies two `Digit` values together and returns the product as a `Digit`.
impl Mul<Digit> for Digit {
    type Output = Digit;

    fn mul(self, other: Digit) -> Self::Output {
        self.txnor(other)
    }
}
/// Divides one `Digit` value by another and returns the result as a `Digit`.
impl Div<Digit> for Digit {
    type Output = Digit;

    fn div(self, other: Digit) -> Self::Output {
        self.tdiv(other).expect("Cannot divide by zero.")
    }
}

/// Adds two `Digit` values together and returns a `Digit` result.
impl Add<Digit> for Digit {
    type Output = DigitResult;

    fn add(self, other: Self) -> Self::Output {
        self.half_adder(other)
    }
}

/// Subtracts two `Digit` values and returns a `Digit` result.
impl Sub<Digit> for Digit {
    type Output = DigitResult;

    fn sub(self, other: Digit) -> Self::Output {
        self.half_adder(-other)
    }
}

impl Ord for Digit {//实现 Ord trait 的类型,以支持 >、<、>=、<= 操作
    fn cmp(&self, other: &Self) -> Ordering {
        match self.tcmp(*other) {
            Digit::Z => Ordering::Equal,//当前对象等于目标对象
            Digit::P => Ordering::Greater, //当前对象大于目标对象
            Digit::N => Ordering::Less,//当前对象小于目标对象
        }
    }
}
impl PartialOrd for Digit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
