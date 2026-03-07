// use super::trit_ops::TritOps;

// #[derive(Clone, Copy, Debug)]
// pub struct Trit16(u32);

// impl Trit16 {
//     pub fn new(v: u32) -> Self {
//         Trit16(v)
//     }
// }

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