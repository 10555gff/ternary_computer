const DECODE: [char;4]=['0','1','T','X'];

pub fn fmt(word: u8) -> String {
    let t0 = DECODE[(word & 0x03) as usize];
    let t1 = DECODE[((word >> 2) & 0x03) as usize];
    let t2 = DECODE[((word >> 4) & 0x03) as usize];
    let t3 = DECODE[((word >> 6) & 0x03) as usize];

    format!("{},{},{},{}", t3, t2, t1, t0)
}

pub fn set_u8(word: u8, n: usize, value: u8) -> u8 {
    let shift = n << 1;
    let mask = 0x03 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}
pub fn read_u8(word: u8, n: usize) -> u8 {
    (word >> (n << 1)) & 0x03
}
// fn clear_u8(word: u8, n: usize) -> u8 {
//     word & !(0x03 << (n << 1))
// }
// fn toggle_u8(word: u8, n: usize) -> u8 {
//     word ^ (0x03 << (n << 1))
// }
// fn swap_u8(word: u8, n: usize) -> u8 {
//     let shift = n << 1;
//     let p = word >> shift;
//     word ^ (((p ^ (p >>1)) & 1)*3 << shift)
// }


pub fn set_u16(word: u16, n: usize, value: u16) -> u16 {
    let shift = n << 1;
    let mask = 0x03u16 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}

pub fn read_u16(word: u16, n: usize) -> u16 {
    (word >> (n << 1)) & 0x03
}