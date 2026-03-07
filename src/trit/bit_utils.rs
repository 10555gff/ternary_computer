const DECODE: [char;4]=['0','1','T','X'];

pub fn fmt(word: u8) -> String {
    let t0 = DECODE[(word & 0x03) as usize];
    let t1 = DECODE[((word >> 2) & 0x03) as usize];
    let t2 = DECODE[((word >> 4) & 0x03) as usize];
    let t3 = DECODE[((word >> 6) & 0x03) as usize];
    format!("{}{}{}{}", t3, t2, t1, t0)
}

pub fn set_2bit_u8(word: u8, n: usize, value: u8) -> u8 {
    let shift = n << 1;
    let mask = 0x03 << shift;
    (word & !mask) | ((value & 0x03) << shift)
}
pub fn read_2bit_u8(word: u8, n: usize) -> u8 {
    (word >> (n << 1)) & 0x03
}

pub fn set_2bit_u16(word: u16, n: usize, value: u8) -> u16 {
    let shift = n << 1;
    let mask = 0x03u16 << shift;
    (word & !mask) | (((value & 0x03)as u16) << shift)
}

pub fn read_2bit_u16(word: u16, n: usize) -> u8 {
    ((word >> (n << 1)) & 0x03) as u8
}



// fn set_2bit_u64(word: u64, n: usize, value: u64) -> u64 {
//     let shift = n << 1;
//     let mask = 0x03u64 << shift;
//     (word & !mask) | ((value & 0x03) << shift)
// }

// fn read_2bit_u64(word: u64, n: usize) -> u8 {
//     ((word >> (n << 1)) & 0x03) as u8
// }