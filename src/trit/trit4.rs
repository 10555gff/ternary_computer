use std::fmt;
use super::bit_utils::*;
use super::trit_ops::TritOps;

#[derive(Clone, Copy, Debug)]
pub struct Trit4(pub u8);


impl TritOps for Trit4 {
    type Output = u8;
    fn get(&self, n:usize)->u8 { read_u8(self.0,n) }
    fn set(&mut self,n:usize,v:u8){ self.0 = set_u8(self.0,n,v) }
}

impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=fmt(self.0);
        write!(f, "Trit4[{}]", val)
    }
}