use std::fmt;
use std::ops::{Shl,Shr};
use core::ops::{Neg, Not, BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign};
use super::logical_table::{TFULLSUM,TFULLCONS};

// 定义位掩码常量，增加可读性
const MASK_EVEN: u8 = 0x55; // 01010101b (c0 位)
const MASK_ODD:  u8 = 0xAA; // 10101010b (c1 位)
const SHIFT: [u8;4] = [0,2,4,6];
const MASK:  [u8;4] = [0x03,0x0C,0x30,0xC0];
const DECODE: [char;4]=['0','1','T','X'];

fn set_2bit(word: u8, k: usize, value: u8) -> u8 {
    let shift = k << 1;
    let mask = 0x03 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}
fn read_2bit(word:u8,k:usize)->u8{
    (word & MASK[k]) >> SHIFT[k]
}
fn clear_2bit(word: u8, k: usize) -> u8 {
    word & !MASK[k]
}
fn toggle_2bit(word: u8, k: usize) -> u8 {
    word ^ MASK[k]
}

fn read_all(word: u8) -> [u8; 4] {
    [
        word & 0x03,
        (word >> 2) & 0x03,
        (word >> 4) & 0x03,
        (word >> 6) & 0x03,
    ]
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trit4(pub u8); // 包装一个 u8

impl Trit4 {
    pub const ZERO: Self = Trit4(0x00);
    pub const POS:  Self = Trit4(0x01);
    pub const NEG:  Self = Trit4(0x02);
    pub const ALL_POS: Self = Trit4(0x55);
    pub const ALL_NEG: Self = Trit4(0xAA);

    pub fn get_all(&self)->[u8; 4] { read_all(self.0) }
    pub fn get(&self, i:usize)->u8 { read_2bit(self.0,i) }
    pub fn clear(&self, i:usize)->u8 { clear_2bit(self.0,i) }
    pub fn toggle(&self, i:usize)->u8 { toggle_2bit(self.0,i) }
    pub fn set(&mut self,i:usize,v:u8){
        self.0 = set_2bit(self.0,i,v)
    }

    #[inline(always)]
    fn or_and(self, other: Self) -> (u8, u8) {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        (or, and)
    }

    #[inline(always)]
    pub fn gate_core(&self, other: Self, code:u8)-> Self{
        let (or, and) = self.or_and(other);
        let mut res:u8=0;

        match code{
            0=>res=(or & MASK_EVEN) | (and & MASK_ODD),//tor
            1=>res=(and & MASK_EVEN) | (or & MASK_ODD),//tand
            2=>res=((or & MASK_EVEN) << 1) | ((and & MASK_ODD) >> 1),//tnor
            3=>res=((and & MASK_EVEN) << 1) | ((or & MASK_ODD) >> 1),//tnand
            4=>res=((or & (or >> 1)) & MASK_EVEN) | ((and | (and << 1)) & MASK_ODD),//txor
            5=>res=((and | (and >> 1)) & MASK_EVEN) | ((or & (or << 1)) & MASK_ODD),//tnxor
            6=>res=and,//tcons
            7=>res=((and >> 1) & MASK_EVEN) | ((and << 1) & MASK_ODD),//tncons
            8 => {
                let m = (or & (or >> 1)) & MASK_EVEN;
                res = or & !(m | (m << 1));
            },//tany
            9 => {
                let nor = !or;
                let m = (nor & (nor >> 1)) & MASK_EVEN;
                res = nor & !(m | (m << 1));
            },//tnany
            _ =>println!("undefine"),
        }
        Trit4(res)
    }

    pub fn shl_trit(self, n: usize) -> Self {
        if n >= 4 { return Trit4(0); }
        Trit4((self.0 << (n << 1)) & 0xFF)
    }
    pub fn shr_trit(self, n: usize) -> Self {
        if n >= 4 { return Trit4(0); }
        Trit4((self.0 >> (n << 1)) & 0xFF)
    }

    pub fn tneg(&self) -> Self {
        let val = self.0;
        let res=((val & 0xAA) >> 1) | ((val & 0x55) << 1);
        Trit4(res)
    }

    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a | b) & 0x55) | ((a & b) & 0xAA);
        Trit4(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a & b) & 0x55) | ((a | b) & 0xAA);
        Trit4(res)
    }

    pub fn tnor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((and & 0xAA) >> 1) | ((or & 0x55) << 1);
        Trit4(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((or & 0xAA) >> 1) | ((and & 0x55) << 1);
        Trit4(res)
    }

    pub fn txor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((and | (and << 1)) & 0xAA) | ((or & (or >> 1)) & 0x55);
        Trit4(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((or & (or << 1)) & 0xAA) | ((and | (and >> 1)) & 0x55);
        Trit4(res)
    }

    pub fn tcons(self, other: Self) -> Self {
        Trit4(self.0 & other.0)
    }
    pub fn tncons(self, other: Self) -> Self {
        let val = self.0 & other.0;
        let res=((val >> 1) & 0x55) | ((val << 1) & 0xAA);
        Trit4(res)
    }

    pub fn tany(self, other: Self) -> Self {
        let or:u8 =self.0 | other.0;
        let mask = (or & (or >> 1)) & 0x55;
        let res = or & !(mask | (mask << 1));
        Trit4(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor:u8 = !(self.0 | other.0);
        let mask = (nor & (nor >> 1)) & 0x55;
        let res = nor & !(mask | (mask << 1));
        Trit4(res)
    }

    pub fn tsum(self, other: Self) -> Self {
        let (o, a) = self.or_and(other);
        let c = ((a & 0x55) << 1) | ((a & 0xAA) >> 1);
        let res = o ^ a ^ c;
        let m = (res & (res >> 1)) & 0x55;
        Trit4(res & !(m | (m << 1)))
    }

    pub fn adder(&self, other: Trit4, mut carry: u8) -> (u8, Trit4) {
        let mut sum = 0;
        for i in 0..4 {
            let shift = i << 1;
            let a = (self.0 >> shift) & 0x03;
            let b = (other.0 >> shift) & 0x03;
            let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as u8;
            carry = TFULLCONS[a as usize][b as usize][carry as usize];
            sum |= sum_i << shift;
        }
        (carry , Trit4(sum))
    }

}


impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 提取 4 个 trit
        let t0 = DECODE[self.get(0) as usize];
        let t1 = DECODE[self.get(1) as usize];
        let t2 = DECODE[self.get(2) as usize];
        let t3 = DECODE[self.get(3) as usize];
        
        // 格式化为 [t3, t2, t1, t0] (高位在前符合数值习惯)
        write!(f, "Trit4[{}, {}, {}, {}]", t3, t2, t1, t0)
    }
}

impl Shl<usize> for Trit4 {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        self.shl_trit(rhs)
    }
}
impl Shr<usize> for Trit4 {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        self.shr_trit(rhs)
    }
}



impl Neg for Trit4 {
    type Output = Trit4;
    fn neg(self) -> Self::Output {
        self.tneg()
    }
}
impl Not for Trit4 {
    type Output = Trit4;
    fn not(self) -> Self::Output {
        self.tneg()
    }
}

impl BitAndAssign for Trit4 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.tand(rhs);
    }
}
impl BitOrAssign for Trit4 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.tor(rhs);
    }
}
impl BitXorAssign for Trit4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.txor(rhs);
    }
}

impl BitAnd<Trit4> for Trit4 {
    type Output = Trit4;

    fn bitand(self, rhs: Trit4) -> Self::Output {
        self.tand(rhs)
    }
}
impl BitOr<Trit4> for Trit4 {
    type Output = Trit4;

    fn bitor(self, rhs: Trit4) -> Self::Output {
        self.tor(rhs)
    }
}
impl BitXor<Trit4> for Trit4 {
    type Output = Trit4;

    fn bitxor(self, rhs: Trit4) -> Self::Output {
        self.txor(rhs)
    }
}