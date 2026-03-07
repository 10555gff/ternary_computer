use super::trit_ops::TritOps;
use super::bit_utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Trit4(u8);

impl Trit4 {
    pub const fn new(v: u8) -> Self {
        Trit4(v)
    }
}

impl TritOps for Trit4 {
    type Output = u8;
    fn get(&self, n:usize)->u8 { read_u8(self.0,n) }
    fn set(&mut self,n:usize,v:u8){ self.0 = set_u8(self.0,n,v) }
}

