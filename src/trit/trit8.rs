use std::fmt;
use super::bit_utils::*;
use super::trit_ops::TritOps;

#[derive(Clone, Copy, Debug)]
pub struct Trit8(pub u16);

impl TritOps for Trit8 {
    type Output = u16;
    fn get(&self, n:usize)->u16 { read_u16(self.0,n) }
    fn set(&mut self,n:usize,v:u16){ self.0 = set_u16(self.0,n,v) }
}

impl fmt::Display for Trit8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=self.0.to_le_bytes();
        let le = fmt(val[0]);
        let be = fmt(val[1]);
        write!(f, "Trit8[{},{}]", be,le)
    }
}