use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor, Add, Sub};
use core::cmp::{Ordering, PartialOrd};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trit32(pub u64);
const MASK_EVEN: u64 = 0x5555_5555_5555_5555;
const MASK_ODD:  u64 = 0xAAAA_AAAA_AAAA_AAAA;

#[derive(Clone, Copy, Debug)]
pub struct TritResult {
    pub carry: u8,
    pub sum: Trit32,
}

impl Trit32 {
    pub const ZERO: Self = Trit32(0x0000_0000_0000_0000);
    pub const POS:  Self = Trit32(0x0000_0000_0000_0001);
    pub const NEG:  Self = Trit32(0x0000_0000_0000_0002);
    pub const ALL_POS: Self = Trit32(0x5555_5555_5555_5555);
    pub const ALL_NEG: Self = Trit32(0xAAAA_AAAA_AAAA_AAAA);

    pub fn get(&self, n:usize) ->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize) ->u64 { TritOps::clear_2bit(self.0,n) }
    pub fn toggle(&self, n:usize)->u64 { TritOps::toggle_2bit(self.0,n) }
    pub fn swap(&self, n:usize)  ->u64 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }
    pub fn leading_zero_trits(self) -> u32 { self.0.leading_zeros() >> 1 }

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res = ((val & MASK_ODD) >> 1) | ((val & MASK_EVEN) << 1);
        Trit32(res)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a & b) & MASK_ODD) | ((a | b) & MASK_EVEN);
        Trit32(res)
    }
    pub fn tnor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and & MASK_ODD) >> 1) | ((or & MASK_EVEN) << 1);
        Trit32(res)
    }
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a | b) & MASK_ODD) | ((a & b) & MASK_EVEN);
        Trit32(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1);
        Trit32(res)
    }
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & MASK_ODD) | ((or & (or >> 1)) & MASK_EVEN);
        Trit32(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & MASK_ODD) | ((and | (and >> 1)) & MASK_EVEN);
        Trit32(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit32(self.0 & other.0)
    }
    pub fn tncons(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = ((and << 1) & MASK_ODD) | ((and >> 1) & MASK_EVEN);
        Trit32(res)
    }
    pub fn tany(self, other: Self) -> Self {
        let or= self.0 | other.0;
        let res = or ^ (((or & (or >> 1)) & MASK_EVEN) * 3);
        Trit32(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor= !(self.0 | other.0);
        let res = nor ^ (((nor & (nor >> 1)) & MASK_EVEN) * 3);
        Trit32(res)
    }
    pub fn tsum(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = self.0 ^ other.0 ^ (((and & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1));
        Trit32(res ^ (((res & (res >> 1)) & MASK_EVEN) * 3))
    }
    pub fn adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (s, c) = TritOps::adder(self.0, other.0, carry);
        (Trit32(s), c)
    }
    pub fn parall_adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (prc, c) = TritOps::tcons3(self.0, other.0, carry);

        let first_sum = self.tsum(other);
        let second_sum = first_sum.tsum(Self(prc));
        (second_sum, c)
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
        let mut index =(31 - n1) as usize;

        let mut remainder=self;
        let mut divisor= divisor_in << fixed;

        let mut fir_quotient = Self::ZERO ;
        let mut sec_quotient = Self::ZERO;
        
        for _ in 0..=fixed{
            let nxor=remainder.tnxor(divisor);
            let digit=nxor.get(index);

            fir_quotient.set(index, digit);
            remainder.update_remainder(digit, divisor);

            if remainder.get(index) != 0 {
                sec_quotient.set(index, digit);
                remainder.update_remainder(digit, divisor);
            }
            
            index-= 1;
            divisor = divisor >> 1;
        }
        index+= 1;
        let final_quotient = (fir_quotient + sec_quotient).sum >>index;
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
            7 => self.tncons(other),// NCONS
            8 => self.tany(other),  // ANY
            9 => self.tsum(other),  // SUM
            _ => unsafe { core::hint::unreachable_unchecked() }
        }
    }

}

impl fmt::Display for Trit32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=self.0.to_le_bytes();
        let t0 = fmt(val[0]);
        let t1 = fmt(val[1]);
        let t2 = fmt(val[2]);
        let t3 = fmt(val[3]);
        let t4 = fmt(val[4]);
        let t5 = fmt(val[5]);
        let t6 = fmt(val[6]);
        let t7 = fmt(val[7]);

        write!(f, "Trit32[{}{}_{}{}_{}{}_{}{}]",t7,t6,t5,t4,t3,t2,t1,t0)
    }
}
impl fmt::Display for TritResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "carry: {}, sum: {}", self.carry, self.sum)
    }
}

impl Shl<usize> for Trit32 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        if rhs >= 32 { return Trit32::ZERO; }
        Trit32(self.0 << (rhs * 2))
    }
}

impl Shr<usize> for Trit32 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        if rhs >= 32 { return Trit32::ZERO; }
        Trit32(self.0 >> (rhs * 2))
    }
}

impl Neg for Trit32 {
    type Output = Trit32;
    fn neg(self) -> Self::Output {
        self.tneg()
    }
}

impl Not for Trit32 {
    type Output = Trit32;
    fn not(self) -> Self::Output {
        self.tneg()
    }
}

impl BitAnd<Trit32> for Trit32 {
    type Output = Trit32;

    fn bitand(self, rhs: Trit32) -> Self::Output {
        self.tand(rhs)
    }
}

impl BitOr<Trit32> for Trit32 {
    type Output = Trit32;

    fn bitor(self, rhs: Trit32) -> Self::Output {
        self.tor(rhs)
    }
}

impl BitXor<Trit32> for Trit32 {
    type Output = Trit32;

    fn bitxor(self, rhs: Trit32) -> Self::Output {
        self.txor(rhs)
    }
}

impl Add<Trit32> for Trit32 {
    type Output = TritResult;

    fn add(self, rhs: Trit32) -> Self::Output {
        let (s, carry) = TritOps::adder(self.0, rhs.0, 0);
        TritResult { carry, sum: Trit32(s) }
    }
}

impl Sub<Trit32> for Trit32 {
    type Output = TritResult;

    fn sub(self, rhs: Trit32) -> Self::Output {
        let other = -rhs;
        let (s, carry) = TritOps::adder(self.0, other.0, 0);
        TritResult { carry, sum: Trit32(s) }
    }
}

impl PartialOrd for Trit32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trit32 {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..32).rev() {
            let a = self.get(i);
            let b = other.get(i);

            if a != b {
                return val(a).cmp(&val(b));
            }
        }
        Ordering::Equal
    }
}
