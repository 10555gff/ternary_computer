use super::trit_ops::TritOps;

fn set_2bit(word: u8, n: usize, value: u8) -> u8 {
    let shift = n << 1;
    let mask = 0x03 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}
fn read_2bit(word: u8, n: usize) -> u8 {
    (word >> (n << 1)) & 0x03
}



#[derive(Clone, Copy, Debug)]
pub struct Trit4(u8);

impl Trit4 {
    pub const fn new(v: u8) -> Self {
        Trit4(v)
    }
}

impl TritOps for Trit4 {
    type Ttype = u8;
    fn get(&self, n:usize)->u8 { read_2bit(self.0,n) }
    fn set(&mut self,n:usize,v:u8){ self.0 = set_2bit(self.0,n,v) }
}

