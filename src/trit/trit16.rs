use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor, Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Trit16(pub u32);
const MASK_EVEN: u32 = 0x5555_5555;
const MASK_ODD:  u32 = 0xAAAA_AAAA;

#[derive(Clone, Copy, Debug)]
pub struct TritResult {
    pub carry: u8,
    pub sum: Trit16,
}

impl Trit16 {
    pub const ZERO: Self = Trit16(0x0000_0000);
    pub const POS:  Self = Trit16(0x0000_0001);
    pub const NEG:  Self = Trit16(0x0000_0002);
    pub const ALL_POS: Self = Trit16(0x5555_5555);
    pub const ALL_NEG: Self = Trit16(0xAAAA_AAAA);

    pub fn get(&self, n:usize) ->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize) ->u32 { TritOps::clear_2bit(self.0,n) }
    pub fn toggle(&self, n:usize)->u32 { TritOps::toggle_2bit(self.0,n) }
    pub fn swap(&self, n:usize)  ->u32 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res = ((val & MASK_ODD) >> 1) | ((val & MASK_EVEN) << 1);
        Trit16(res)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a & b) & MASK_ODD) | ((a | b) & MASK_EVEN);
        Trit16(res)
    }
    pub fn tnor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and & MASK_ODD) >> 1) | ((or & MASK_EVEN) << 1);
        Trit16(res)
    }
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a | b) & MASK_ODD) | ((a & b) & MASK_EVEN);
        Trit16(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1);
        Trit16(res)
    }
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & MASK_ODD) | ((or & (or >> 1)) & MASK_EVEN);
        Trit16(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & MASK_ODD) | ((and | (and >> 1)) & MASK_EVEN);
        Trit16(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit16(self.0 & other.0)
    }
    pub fn tncons(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = ((and << 1) & MASK_ODD) | ((and >> 1) & MASK_EVEN);
        Trit16(res)
    }
    pub fn tany(self, other: Self) -> Self {
        let or= self.0 | other.0;
        let res = or ^ (((or & (or >> 1)) & MASK_EVEN) * 3);
        Trit16(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor= !(self.0 | other.0);
        let res = nor ^ (((nor & (nor >> 1)) & MASK_EVEN) * 3);
        Trit16(res)
    }
    pub fn tsum(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = self.0 ^ other.0 ^ (((and & MASK_ODD) >> 1) | ((and & MASK_EVEN) << 1));
        Trit16(res ^ (((res & (res >> 1)) & MASK_EVEN) * 3))
    }
    pub fn adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (s, c) = TritOps::adder(self.0, other.0, carry);
        (Trit16(s), c)
    }
    pub fn parall_adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (prc, c) = TritOps::tcons3(self.0, other.0, carry);

        let first_sum = self.tsum(other);
        let second_sum = first_sum.tsum(Self(prc));
        (second_sum, c)
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

impl fmt::Display for Trit16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=self.0.to_le_bytes();
        let t0 = fmt(val[0]);
        let t1 = fmt(val[1]);
        let t2 = fmt(val[2]);
        let t3 = fmt(val[3]);

        write!(f, "Trit16[{}{}_{}{}]",t3,t2,t1,t0)
    }
}
impl fmt::Display for TritResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "carry: {}, sum: {}", self.carry, self.sum)
    }
}

impl Shl<usize> for Trit16 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        if rhs >= 16 { return Trit16::ZERO; }
        Trit16(self.0 << (rhs << 1))
    }
}

impl Shr<usize> for Trit16 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        if rhs >= 16 { return Trit16::ZERO; }
        Trit16(self.0 >> (rhs << 1))
    }
}

impl Neg for Trit16 {
    type Output = Trit16;
    fn neg(self) -> Self::Output {
        self.tneg()
    }
}

impl Not for Trit16 {
    type Output = Trit16;
    fn not(self) -> Self::Output {
        self.tneg()
    }
}

impl BitAnd<Trit16> for Trit16 {
    type Output = Trit16;

    fn bitand(self, rhs: Trit16) -> Self::Output {
        self.tand(rhs)
    }
}

impl BitOr<Trit16> for Trit16 {
    type Output = Trit16;

    fn bitor(self, rhs: Trit16) -> Self::Output {
        self.tor(rhs)
    }
}

impl BitXor<Trit16> for Trit16 {
    type Output = Trit16;

    fn bitxor(self, rhs: Trit16) -> Self::Output {
        self.txor(rhs)
    }
}

impl Add<Trit16> for Trit16 {
    type Output = TritResult;

    fn add(self, rhs: Trit16) -> Self::Output {
        let (s, carry) = TritOps::adder(self.0, rhs.0, 0);
        TritResult { carry, sum: Trit16(s) }
    }
}

impl Sub<Trit16> for Trit16 {
    type Output = TritResult;

    fn sub(self, rhs: Trit16) -> Self::Output {
        let other = -rhs;
        let (s, carry) = TritOps::adder(self.0, other.0, 0);
        TritResult { carry, sum: Trit16(s) }
    }
}
