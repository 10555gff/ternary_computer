use std::fmt;
use super::bit_utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Trit16(pub u32);

impl Trit16 {
    pub fn get(&self, n:usize) ->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize) ->u32 { TritOps::clear_2bit(self.0,n) }
    pub fn toggle(&self, n:usize)->u32 { TritOps::toggle_2bit(self.0,n) }
    pub fn tneg(&self, n:usize)  ->u32 { TritOps::swap_2bit(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 =TritOps::set_2bit(self.0,n,v) }

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