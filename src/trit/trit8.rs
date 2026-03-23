use std::fmt;
use super::bit_utils::*;
use core::ops::{Shl, Shr};

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
    pub fn tneg(&self, n:usize)  ->u16 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }

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