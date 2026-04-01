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
    pub fn tmax3(self, b: Self, c: Self) -> Self {
        self.tor(b).tor(c)
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
    pub fn tmin3(self, b: Self, c: Self) -> Self {
        self.tand(b).tand(c)
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
    pub fn tany(self, other: Self) -> Self {
        let or = self.0 | other.0;
        let res = or ^ (((or & (or >> 1)) & 0x5555) * 3);
        Trit8(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor= !(self.0 | other.0);
        let res = nor ^ (((nor & (nor >> 1)) & 0x5555) * 3);
        Trit8(res)
    }
    pub fn tsum(self, other: Self) -> Self {
        let and = self.0 & other.0;
        let res = self.0 ^ other.0 ^ (((and & 0x5555) << 1) | ((and & 0xAAAA) >> 1));
        Trit8(res ^ (((res & (res >> 1)) & 0x5555) * 3))
    }
    pub fn adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (s, c) = TritOps::adder(self.0, other.0, carry);
        (Trit8(s), c)
    }
    pub fn parall_adder(self, other: Self, carry: u8) -> (Self, u8) {
        let (prc, c) = TritOps::tcons3(self.0, other.0, carry);

        let first_sum = self.tsum(other);
        let second_sum = first_sum.tsum(Self(prc));
        (second_sum, c)
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
