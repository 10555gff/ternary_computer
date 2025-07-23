pub mod logical_table;
use core::ops::{Neg, Not, BitOr, BitAnd, BitXor, Add, Sub, Mul, Div};

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
    logical_table::TOR[self as usize][other as usize]
}
pub const fn tand(self, other: Self) -> Self {
    logical_table::TAND[self as usize][other as usize]
}
pub const fn tnor(self, other: Self) -> Self {
    logical_table::TNOR[self as usize][other as usize]
}
pub const fn tnand(self, other: Self) -> Self {
    logical_table::TNAND[self as usize][other as usize]
}
pub const fn txor(self, other: Self) -> Self {
    logical_table::TXOR[self as usize][other as usize]
}
pub const fn txnor(self, other: Self) -> Self {
    logical_table::TXNOR[self as usize][other as usize]
}
pub const fn tsum(self, other: Self) -> Self {
    logical_table::TSUM[self as usize][other as usize]
}
pub const fn tcons(self, other: Self) -> Self {
    logical_table::TCONS[self as usize][other as usize]
}
pub const fn tany(self, other: Self) -> Self {
    logical_table::TANY[self as usize][other as usize]
}
pub const fn tpoz(self, other: Self) -> Self {
    logical_table::TPOZ[self as usize][other as usize]
}
pub const fn tcmp(self, other: Self) -> Self {
    logical_table::TCMP[self as usize][other as usize]
}
pub const fn tdiv(self, other: Self) -> Option<Self> {
    logical_table::TDIV[self as usize][other as usize]
}
pub const fn t3or(self, b: Self,c: Self) -> Self {
    logical_table::T3OR[self as usize][b as usize][c as usize]
}
pub const fn t3and(self, b: Self,c: Self) -> Self {
    logical_table::T3AND[self as usize][b as usize][c as usize]
}

pub const fn half_adder(self, other: Self) -> DigitResult {
    let sum = logical_table::TSUM[self as usize][other as usize];// 和
    let carry=logical_table::TCONS[self as usize][other as usize];// 进位;
    DigitResult { carry, sum }
}
pub const fn full_adder(self, b: Self,c_in: Self) -> DigitResult {
    let sum =logical_table::TFULLSUM[self as usize][b as usize][c_in as usize];// 和
    let carry=logical_table::TFULLCONS[self as usize][b as usize][c_in as usize];// 进位
    DigitResult { carry, sum }
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


