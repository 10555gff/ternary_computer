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
    fn toggle_2bit(word: Self, n: usize) -> Self;
    fn swap_2bit(word: Self, n: usize) -> Self;
    fn set_2bit(word: Self, n: usize, value: u8) -> Self;
}

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
            fn toggle_2bit(word: $t, n: usize) -> $t {
                let mask = (0x03 as $t) << (n << 1);
                word ^ mask 
            }
            fn swap_2bit(word: $t, n: usize) -> $t {
                let shift = n << 1;
                let val = word >> shift;
                let mask = (((val ^ (val >> 1)) & 1) * 3) as $t;
                word ^ (mask << shift)
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
impl_trit_ops_for!(u32);
impl_trit_ops_for!(u64);


