use std::fmt;
use super::bit_utils::*;
// use super::trit_ops::TritOps;

#[derive(Clone, Copy, Debug)]
pub struct Trit32(pub u64);


// impl TritOps for Trit32 {

//     fn get(&self, n: usize) -> u8 {
//         ((self.0 >> (n * 2)) & 0b11) as u8
//     }

//     fn set(&mut self, n: usize, val: u8) {

//         let shift = n * 2;
//         let mask = 0b11 << shift;

//         self.0 =0;
//     }

// }

impl fmt::Display for Trit32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val=self.0.to_le_bytes();
        let t0 = fmt(val[0]);
        let t1 = fmt(val[1]);
        let t2 = fmt(val[2]);
        let t3 = fmt(val[3]);
        let t4 = fmt(val[4]);
        let t5 = fmt(val[5]);
        let t6 = fmt(val[6]);
        let t7 = fmt(val[7]);

        write!(f, "Trit32[{},{},{},{},{},{},{},{}]",t7,t6,t5,t4,t3,t2,t1,t0)
    }
}