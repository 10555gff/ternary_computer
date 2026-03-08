use std::fmt;
use super::bit_utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Trit8(pub u16);

impl Trit8 {
    pub fn get(&self, n:usize)->u8 { TritOps::read_2bit(self.0,n) }
    pub fn clear(&self, n:usize)->u16 { TritOps::clear_2bit(self.0,n) }
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