use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor};

#[derive(Clone, Copy, Debug)]
pub struct Trit4(pub u8);

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

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res=((val & 0xAA) >> 1) | ((val & 0x55) << 1);
        Trit4(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit4(self.0 & other.0)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a & b) & 0xAA) | ((a | b) & 0x55);
        Trit4(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a | b) & 0xAA) | ((a & b) & 0x55);
        Trit4(res)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & 0xAA) | ((or & (or >> 1)) & 0x55);
        Trit4(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & 0xAA) | ((and | (and >> 1)) & 0x55);
        Trit4(res)
    }
    
}

impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=fmt(self.0);
        write!(f, "Trit4[{}]", val)
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
