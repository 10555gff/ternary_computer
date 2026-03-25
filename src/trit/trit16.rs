use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr};

#[derive(Clone, Copy, Debug)]
pub struct Trit16(pub u32);

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
        let res = ((val & 0xAAAA_AAAA) >> 1) | ((val & 0x5555_5555) << 1);
        Trit16(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit16(self.0 & other.0)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a & b) & 0xAAAA_AAAA) | ((a | b) & 0x5555_5555);
        Trit16(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a | b) & 0xAAAA_AAAA) | ((a & b) & 0x5555_5555);
        Trit16(res)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & 0xAAAA_AAAA) | ((or & (or >> 1)) & 0x5555_5555);
        Trit16(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & 0xAAAA_AAAA) | ((and | (and >> 1)) & 0x5555_5555);
        Trit16(res)
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