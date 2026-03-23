use std::fmt;
use super::bit_utils::*;

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
    pub fn tneg(&self, n:usize)  ->u64 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }

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