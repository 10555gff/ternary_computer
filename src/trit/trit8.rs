use super::trit_ops::TritOps;

fn set_2bit(word: u16, n: usize, value: u16) -> u16 {
    let shift = n << 1;
    let mask = 0x03u16 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}

fn read_2bit(word: u16, n: usize) -> u16 {
    (word >> (n << 1)) & 0x03
}



#[derive(Clone, Copy, Debug)]
pub struct Trit8(u16);

impl Trit8 {
    pub fn new(v: u16) -> Self {
        Trit8(v)
    }
}

impl TritOps for Trit8 {
    type Ttype = u16;
    fn get(&self, n:usize)->u16 { read_2bit(self.0,n) }
    fn set(&mut self,n:usize,v:u16){ self.0 = set_2bit(self.0,n,v) }
}

