use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor};

#[derive(Clone, Copy, Debug)]
pub struct Trit8(pub u16);

impl Trit8 {
    pub const ZERO: Self = Trit8(0x0000);
    pub const POS:  Self = Trit8(0x0001);
    pub const NEG:  Self = Trit8(0x0002);
    pub const ALL_POS: Self = Trit8(0x5555);
    pub const ALL_NEG: Self = Trit8(0xAAAA);

    pub fn get(&self, n:usize) ->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize) ->u16 { TritOps::clear_2bit(self.0,n) }
    pub fn toggle(&self, n:usize)->u16 { TritOps::toggle_2bit(self.0,n) }
    pub fn swap(&self, n:usize)  ->u16 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res = ((val & 0xAAAA) >> 1) | ((val & 0x5555) << 1);
        Trit8(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit8(self.0 & other.0)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a & b) & 0xAAAA) | ((a | b) & 0x5555);
        Trit8(res)
    }
    pub fn tnor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and & 0xAAAA) >> 1) | ((or & 0x5555) << 1);
        Trit8(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a | b) & 0xAAAA) | ((a & b) & 0x5555);
        Trit8(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & 0xAAAA) >> 1) | ((and & 0x5555) << 1);
        Trit8(res)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & 0xAAAA) | ((or & (or >> 1)) & 0x5555);
        Trit8(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & 0xAAAA) | ((and | (and >> 1)) & 0x5555);
        Trit8(res)
    }
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
    }
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
    }
    
    
}

impl fmt::Display for Trit8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=self.0.to_le_bytes();
        let le = fmt(val[0]);
        let be = fmt(val[1]);
        write!(f, "Trit8[{}{}]", be,le)
    }
}
impl Shl<usize> for Trit8 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        if rhs >= 8 { return Trit8::ZERO; }
        Trit8(self.0 << (rhs << 1))
    }
}

impl Shr<usize> for Trit8 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        if rhs >= 8 { return Trit8::ZERO; }
        Trit8(self.0 >> (rhs << 1))
    }
}

impl Neg for Trit8 {
    type Output = Trit8;
    fn neg(self) -> Self::Output {
        self.tneg()
    }
}

impl Not for Trit8 {
    type Output = Trit8;
    fn not(self) -> Self::Output {
        self.tneg()
    }
}

impl BitAnd<Trit8> for Trit8 {
    type Output = Trit8;

    fn bitand(self, rhs: Trit8) -> Self::Output {
        self.tand(rhs)
    }
}

impl BitOr<Trit8> for Trit8 {
    type Output = Trit8;

    fn bitor(self, rhs: Trit8) -> Self::Output {
        self.tor(rhs)
    }
}

impl BitXor<Trit8> for Trit8 {
    type Output = Trit8;

    fn bitxor(self, rhs: Trit8) -> Self::Output {
        self.txor(rhs)
    }
}
