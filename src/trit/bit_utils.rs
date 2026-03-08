const DECODE: [char;4]=['0','1','T','X'];

pub fn fmt(word: u8) -> String {
    let t0 = DECODE[(word & 0x03) as usize];
    let t1 = DECODE[((word >> 2) & 0x03) as usize];
    let t2 = DECODE[((word >> 4) & 0x03) as usize];
    let t3 = DECODE[((word >> 6) & 0x03) as usize];
    format!("{}{}{}{}", t3, t2, t1, t0)
}

pub trait TritOps{
    fn read_2bit(word: Self, n: usize) -> u8;
    fn clear_2bit(word: Self, n: usize) -> Self;
    fn set_2bit(word: Self, n: usize, value: u8) -> Self;

}



// pub fn clear_2bit_u8(word: u8, n: usize) -> u8 {
//     word & !(0x03 << (n << 1))
// }

// pub fn clear_2bit_u16(word: u16, n: usize) -> u16 {
//     let mask = (0x03 as u16) << (n << 1);
//     word & !mask
// }

// pub fn set_2bit_u16(word: u16, n: usize, value: u8) -> u16 {
//     let shift = n << 1;
//     let mask = 0x03u16 << shift;
//     (word & !mask) | (((value & 0x03)as u16) << shift)
// }


// pub fn set_2bit_u64(word: u64, n: usize, value: u64) -> u64 {
//     let shift = n << 1;
//     let mask = 0x03u64 << shift;
//     (word & !mask) | ((value & 0x03) << shift)
// }







// fn set_2bit(word: u8, n: usize, value: u8) -> u8 {
//     let shift = n << 1;
//     let mask = 0x03 << shift;
//     (word & !mask) | ((value & 0x03) << shift)
// }
// fn read_2bit(word: u8, n: usize) -> u8 {
//     (word >> (n << 1)) & 0x03
// }
// fn clear_2bit(word: u8, n: usize) -> u8 {
//     word & !(0x03 << (n << 1))
// }
// fn toggle_2bit(word: u8, n: usize) -> u8 {
//     word ^ (0x03 << (n << 1))
// }
// fn swap_2bit(word: u8, n: usize) -> u8 {
//     let shift = n << 1;
//     let p = word >> shift;
//     word ^ (((p ^ (p >>1)) & 1)*3 << shift)
// }




macro_rules! impl_trit_ops_for {
    ($t:ty) => {
        impl TritOps for $t {
            fn read_2bit(word: $t, n: usize) -> u8 {
                ((word >> (n << 1)) & 0x03) as u8
            }
            fn clear_2bit(word: $t, n: usize) -> $t {
                let mask = (0x03 as $t) << (n << 1);
                word & !mask 
            }
            fn set_2bit(word: $t, n: usize, value: u8) -> $t {
                let shift = n << 1;
                let mask = (0x03 as $t) << shift;
                (word & !mask) | (((value & 0x03) as $t) << shift)
            }
            
        }
    };
}
impl_trit_ops_for!(u8);
impl_trit_ops_for!(u16);
// impl_trit_ops_for!(u32);
// impl_trit_ops_for!(u64);


