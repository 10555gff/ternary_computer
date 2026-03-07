// use super::trit_ops::TritOps;

// #[derive(Clone, Copy, Debug)]
// pub struct Trit32(u64);

// impl Trit32 {
//     pub fn new(v: u64) -> Self {
//         Trit32(v)
//     }
// }

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