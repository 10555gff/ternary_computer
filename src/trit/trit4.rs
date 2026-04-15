use std::fmt;
use crate::trit::Trit8;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor, Add, Sub, Mul, Div};
use core::cmp::{Ordering, PartialOrd};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trit4(pub u8);
const MASK_EVEN: u8 = 0x55;
const MASK_ODD:  u8 = 0xAA;

#[derive(Clone, Copy, Debug)]
pub struct TritResult {
    pub carry: u8,
    pub sum: Trit4,
}

#[derive(Clone, Copy, Debug)]
pub struct DivResult {
    pub quotient: Trit4,
    pub remainder: Trit4,
}

impl Trit4 {
    pub const ZERO: Self = Trit4(0x00);
    pub const POS:  Self = Trit4(0x01);
    pub const NEG:  Self = Trit4(0x02);
    pub const ALL_POS: Self = Trit4(0x55);
    pub const ALL_NEG: Self = Trit4(0xAA);

    pub fn get(&self, n:usize) ->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize) ->u8 { TritOps::clear_2bit(self.0,n) }
    pub fn toggle(&self, n:usize) ->u8 { TritOps::toggle_2bit(self.0,n) }
    pub fn swap(&self, n:usize) ->u8 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0=TritOps::set_2bit(self.0,n,v) }

    pub fn to_dec(self) -> i64{ TritOps::to_dec(self.0) }
    pub fn leading_zero_trits(self) -> u32 { self.0.leading_zeros() >> 1 }

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res=((val & MASK_ODD) >> 1) | ((val & MASK_EVEN) << 1);
        Trit4(res)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a & b) & MASK_ODD) | ((a | b) & MASK_EVEN);
        Trit4(res)
    }
    pub fn tnor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and & MASK_ODD) >> 1) | ((or & MASK_EVEN) << 1);
        Trit4(res)
    }
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a | b) & MASK_ODD) | ((a & b) & MASK_EVEN);
        Trit4(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1);
        Trit4(res)
    }
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & MASK_ODD) | ((or & (or >> 1)) & MASK_EVEN);
        Trit4(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & MASK_ODD) | ((and | (and >> 1)) & MASK_EVEN);
        Trit4(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit4(self.0 & other.0)
    }
    pub fn tncons(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = ((and << 1) & MASK_ODD) | ((and >> 1) & MASK_EVEN);
        Trit4(res)
    }
    pub fn tany(self, other: Self) -> Self {
        let or= self.0 | other.0;
        let res = or ^ (((or & (or >> 1)) & MASK_EVEN) * 3);
        Trit4(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor= !(self.0 | other.0);
        let res = nor ^ (((nor & (nor >> 1)) & MASK_EVEN) * 3);
        Trit4(res)
    }
    pub fn tsum(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = self.0 ^ other.0 ^ (((and & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1));
        Trit4(res ^ (((res & (res >> 1)) & MASK_EVEN) * 3))
    }

    pub fn from_dec(mut dec: i8) -> Self {
        if dec == 0 {
            return Trit4::ZERO;
        }
        if !(-40..=40).contains(&dec) {
            panic!("Trit4 range (-40..=40): Invalid value: {}", dec);
        }

        let mut res = Self::ZERO;
        let mut n: usize = 0;

        while dec != 0 {
            let remainder = dec % 3;
            match remainder {
                -1 | 2 => {
                    res.set(n, 0b10);
                    dec = (dec + 1) / 3;
                }
                -2 | 1 => {
                    res.set(n, 0b01);
                    dec = (dec - 1) / 3;
                }
                _ => {
                    dec = dec / 3;
                }
            }
            n += 1;
        }
        res
    }

    pub fn adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (s, c) = TritOps::adder(self.0, other.0, carry);
        (Trit4(s), c)
    }
    pub fn parall_adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (prc, c) = TritOps::tcons3(self.0, other.0, carry);

        let first_sum = self.tsum(other);
        let second_sum = first_sum.tsum(Self(prc));
        (second_sum, c)
    }

    pub fn mul(self, other: Self) -> Trit8 {
        let mut pos_part= Trit8(self.0 as u16);
        let mut neg_part= -pos_part;
        let mut part_product=Trit8::ZERO;

        for i in 0..4 {
            let n = other.get(i);
            if n == 1 {
                part_product = (part_product + pos_part).sum;
            } else if n == 2 {
                part_product = (part_product + neg_part).sum;
            }
            pos_part = pos_part<<1;
            neg_part = neg_part<<1;
        }

        part_product
    }

    fn update_remainder(&mut self, digit: u8, divisor: Self) {
        *self = match digit {
            1 => (*self - divisor).sum,
            2 => (*self + divisor).sum,
            _ => *self,
        };
    }
    pub fn div(self,divisor_in: Self)-> (Self, Self){
        if divisor_in == Self::ZERO {
            panic!("Cannot divide by zero.");
        }

        let n1 = self.leading_zero_trits();
        let n2 = divisor_in.leading_zero_trits();
        if n1 > n2 {
            return (Self::ZERO, self);
        }

        let fixed = (n2 - n1) as usize;
        let mut index =(3 - n1) as usize;

        let mut remainder=self;
        let mut divisor= divisor_in << fixed;

        let mut fir_quotient = Self::ZERO ;
        let mut sec_quotient = Self::ZERO;
        
        for step in 0..=fixed{
            let digit = tnxor_digit(remainder.get(index), divisor.get(index));

            if digit != 0 {
                fir_quotient.set(index, digit);
                remainder.update_remainder(digit, divisor);

                if remainder.get(index) != 0 {
                    sec_quotient.set(index, digit);
                    remainder.update_remainder(digit, divisor);
                }
            }

            if step != fixed {
                index -= 1;
                divisor = divisor >> 1;
            }
        }
        fir_quotient = fir_quotient >> index;
        sec_quotient = sec_quotient >> index;
        let final_quotient = (fir_quotient + sec_quotient).sum;
        (final_quotient, remainder)
    }

    pub fn gate_core(self, other: Self, code: u8) -> Self {
        match code {
            0 => self.tand(other),  // MIN
            1 => self.tor(other),   // MAX
            2 => self.txor(other),  // XOR
            3 => self.tnand(other), // NAND
            4 => self.tnor(other),  // NOR
            5 => self.tnxor(other), // NXOR
            6 => self.tcons(other), // CONS
            7 => self.tany(other),  // ANY
            8 => self.adder(other,0).0,  // SUM
            _ => unsafe { core::hint::unreachable_unchecked() }
        }
    }
    
}

impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=fmt(self.0);
        write!(f, "Trit4[{}]", val)
    }
}
impl fmt::Display for TritResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "carry: {}, sum: {}", self.carry, self.sum)
    }
}
impl fmt::Display for DivResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "quotient: {}, remainder: {}", self.quotient, self.remainder)
    }
}

impl Shl<usize> for Trit4 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        if rhs >= 4 { return Trit4::ZERO; }
        Trit4(self.0 << (rhs << 1))
    }
}

impl Shr<usize> for Trit4 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        if rhs >= 4 { return Trit4::ZERO; }
        Trit4(self.0 >> (rhs << 1))
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

impl Add<Trit4> for Trit4 {
    type Output = TritResult;

    fn add(self, rhs: Trit4) -> Self::Output {
        let (s, carry) = TritOps::adder(self.0, rhs.0, 0);
        TritResult { carry, sum: Trit4(s) }
    }
}

impl Sub<Trit4> for Trit4 {
    type Output = TritResult;

    fn sub(self, rhs: Trit4) -> Self::Output {
        let other = -rhs;
        let (s, carry) = TritOps::adder(self.0, other.0, 0);
        TritResult { carry, sum: Trit4(s) }
    }
}

impl Mul<Trit4> for Trit4 {
    type Output = Trit8;

    fn mul(self, rhs: Trit4) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div<Trit4> for Trit4 {
    type Output = DivResult;

    fn div(self, rhs: Trit4) -> Self::Output {
        let (q,r)=self.div(rhs);
        DivResult { quotient:q, remainder:r }
    }
}

impl PartialOrd for Trit4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trit4 {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..4).rev() {
            let a = self.get(i);
            let b = other.get(i);

            if a != b {
                return val(a).cmp(&val(b));
            }
        }
        Ordering::Equal
    }
}
