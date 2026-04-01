use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr, Neg, Not, BitAnd, BitOr, BitXor};

#[derive(Clone, Copy, Debug)]
pub struct Trit32(pub u64);

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

    pub fn tneg(self) -> Self {
        let val = self.0;
        let res = ((val & 0xAAAA_AAAA_AAAA_AAAA) >> 1)
                | ((val & 0x5555_5555_5555_5555) << 1);
        Trit32(res)
    }
    pub fn tcons(self, other: Self) -> Self {
        Trit32(self.0 & other.0)
    }
    pub fn tor(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a & b) & 0xAAAA_AAAA_AAAA_AAAA)
                | ((a | b) & 0x5555_5555_5555_5555);
        Trit32(res)
    }
    pub fn tnor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and & 0xAAAA_AAAA_AAAA_AAAA) >> 1) 
                   | ((or & 0x5555_5555_5555_5555) << 1);
        Trit32(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res = ((a | b) & 0xAAAA_AAAA_AAAA_AAAA)
                | ((a & b) & 0x5555_5555_5555_5555);
        Trit32(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & 0xAAAA_AAAA_AAAA_AAAA) >> 1) 
                   | ((and & 0x5555_5555_5555_5555) << 1);
        Trit32(res)
    }
    pub fn txor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((and | (and << 1)) & 0xAAAA_AAAA_AAAA_AAAA) 
              | ((or & (or >> 1)) & 0x5555_5555_5555_5555);
        Trit32(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        let res=((or & (or << 1)) & 0xAAAA_AAAA_AAAA_AAAA) 
              | ((and | (and >> 1)) & 0x5555_5555_5555_5555);
        Trit32(res)
    }
    pub fn tany(self, other: Self) -> Self {
        let or= self.0 | other.0;
        let res = or ^ (((or & (or >> 1)) & 0x5555_5555_5555_5555) * 3);
        Trit32(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor= !(self.0 | other.0);
        let res = nor ^ (((nor & (nor >> 1)) & 0x5555_5555_5555_5555) * 3);
        Trit32(res)
    }
    pub fn adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (s, c) = TritOps::adder(self.0, other.0, carry);
        (Trit32(s), c)
    }
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
    }
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
    }
    pub fn gate_core(self, other: Self, code: u8) -> Self {
        match code {
            0 => self.tcons(other), // CONS
            1 => self.tand(other),  // MIN
            2 => self.tor(other),   // MAX
            3 => self.txor(other),  // XOR
            4 => self.tnand(other), // NAND
            5 => self.tnor(other),  // NOR
            6 => self.tnxor(other), // NXOR
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