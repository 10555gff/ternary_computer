use std::fmt;
use super::bit_utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Trit4(pub u8);

impl Trit4 {
    pub fn get(&self, n:usize)->u8 { read_2bit_u8(self.0,n) }
    pub fn set(&mut self,n:usize,v:u8){ self.0 = set_2bit_u8(self.0,n,v) }

}


impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=fmt(self.0);
        write!(f, "Trit4[{}]", val)
    }
}