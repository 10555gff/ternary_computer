use super::trit_ops::TritOps;
use super::bit_utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Trit8(pub u16);

impl TritOps for Trit8 {
    type Output = u16;
    fn get(&self, n:usize)->u16 { read_u16(self.0,n) }
    fn set(&mut self,n:usize,v:u16){ self.0 = set_u16(self.0,n,v) }
}

