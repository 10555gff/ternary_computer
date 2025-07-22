pub mod logical_table;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    Z =0,
    P =1,
    N =2,
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
    let n=logical_table::TOR[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tand(self, other: Self) -> Self {
    let n=logical_table::TAND[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tnor(self, other: Self) -> Self {
    let n=logical_table::TNOR[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tnand(self, other: Self) -> Self {
    let n=logical_table::TNAND[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn txor(self, other: Self) -> Self {
    let n=logical_table::TXOR[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn txnor(self, other: Self) -> Self {
    let n=logical_table::TXNOR[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tsum(self, other: Self) -> Self {
    let n=logical_table::TSUM[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tcons(self, other: Self) -> Self {
    let n=logical_table::TCONS[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tany(self, other: Self) -> Self {
    let n=logical_table::TANY[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tpoz(self, other: Self) -> Self {
    let n=logical_table::TPOZ[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tcmp(self, other: Self) -> Self {
    let n=logical_table::TCMP[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn tdiv(self, other: Self) -> Self {
    let n=logical_table::TDIV[self as usize][other as usize];
    Digit::from_u8(n)
}
pub const fn t3or(self, b: Self,c: Self) -> Self {
    let n=logical_table::T3OR[self as usize][b as usize][c as usize];
    Digit::from_u8(n)
}
pub const fn t3and(self, b: Self,c: Self) -> Self {
    let n=logical_table::T3AND[self as usize][b as usize][c as usize];
    Digit::from_u8(n)
}

pub const fn half_adder(self, other: Self) -> (u8, u8) {
    let sum = logical_table::TSUM[self as usize][other as usize];// 和
    let carry=logical_table::TCONS[self as usize][other as usize];// 进位;
    (carry,sum)
}
pub const fn full_adder(self, b: Self,c_in: Self) -> (u8, u8) {
    let sum =logical_table::TFULLSUM[self as usize][b as usize][c_in as usize];// 和
    let carry=logical_table::TFULLCONS[self as usize][b as usize][c_in as usize];// 进位
    (carry,sum)
}

}


/// Performs a bitwise OR (`|`) operation between two `Digit` values and returns the result.
impl BitOr for Digit {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        let n=logical_table::TOR[self as usize][other as usize];
        Digit::from_u8(n)
    }
}