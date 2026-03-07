use std::fmt;
use super::bit_utils::*;
// use super::trit_ops::TritOps;

#[derive(Clone, Copy, Debug)]
pub struct Trit16(pub u32);

// impl TritOps for Trit16 {

//     fn get(&self, n: usize) -> u8 {
//         ((self.0 >> (n * 2)) & 0b11) as u8
//     }

//     fn set(&mut self, n: usize, val: u8) {

//         let shift = n * 2;
//         let mask = 0b11 << shift;

//         self.0 = (self.0 & !mask) | ((val as u32 & 0b11) << shift);
//     }

// }

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