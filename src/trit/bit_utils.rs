const DECODE: [char;4]=['0','1','T','X'];

const TFULLSUM: [[[u8; 3]; 3]; 3] = [
    [
        [0, 1, 2],
        [1, 2, 0],
        [2, 0, 1],
    ],
    [
        [1, 2, 0],
        [2, 0, 1],
        [0, 1, 2],
    ],
    [
        [2, 0, 1],
        [0, 1, 2],
        [1, 2, 0],
    ],
];
const TFULLCONS: [[[u8; 3]; 3]; 3] = [
    [
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 2],
    ],
    [
        [0, 1, 0],
        [1, 1, 0],
        [0, 0, 0],
    ],
    [
        [0, 0, 2],
        [0, 0, 0],
        [2, 0, 2],
    ],
];

#[inline]
pub fn val(x: u8) -> i8 {
    [0, 1, -1][x as usize]
}

pub fn fmt(word: u8) -> String {
    let t0 = DECODE[(word & 0x03) as usize];
    let t1 = DECODE[((word >> 2) & 0x03) as usize];
    let t2 = DECODE[((word >> 4) & 0x03) as usize];
    let t3 = DECODE[((word >> 6) & 0x03) as usize];
    format!("{}{}{}{}", t3, t2, t1, t0)
}

pub trait TritOps: Sized {
    fn to_dec(word: Self) -> i64;
    fn read_2bit(word: Self, n: usize) -> u8;
    fn clear_2bit(word: Self, n: usize) -> Self;
    fn toggle_2bit(word: Self, n: usize) -> Self;
    fn swap_2bit(word: Self, n: usize) -> Self;
    fn set_2bit(word: Self, n: usize, value: u8) -> Self;
    fn adder(word: Self, other: Self, carry: u8) -> (Self, u8);
    fn tcons3(word: Self, word2: Self, carry: u8) -> (Self, u8);
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

            fn to_dec(word: $t) -> i64 {
                let mut dec = 0i64;
                let mut pow = 1i64;
                let width = core::mem::size_of::<$t>() * 4;

                let mut w = word;
                for _ in 0..width {
                    let trit = val((w & 0x03) as u8) as i64;
                    dec += trit * pow;
                    pow *= 3;
                    w >>= 2;
                }
                dec
            }

            fn tcons3(mut word: $t, mut word2: $t, mut carry: u8) -> ($t, u8) {
                let mut res: $t = 0;
                let mut shift = 0;
                let width = core::mem::size_of::<$t>() * 4;

                for _ in 0..width {
                    let a = (word & 0x03) as usize;
                    let b = (word2 & 0x03) as usize;

                    res |= (carry as $t) << shift;
                    carry = TFULLCONS[a][b][carry as usize];

                    word >>= 2;
                    word2 >>= 2;
                    shift += 2;
                }
                (res, carry)
            }
            fn adder(word: $t, other: $t, mut carry: u8) -> ($t, u8) {
                let mut sum: $t = 0;
                let width = core::mem::size_of::<$t>() * 4;

                let mut a_word = word;
                let mut b_word = other;
                let mut shift = 0;

                for _ in 0..width {
                    let a = (a_word & 0x03) as usize;
                    let b = (b_word & 0x03) as usize;

                    let sum_i = TFULLSUM[a][b][carry as usize] as $t;
                    carry = TFULLCONS[a][b][carry as usize];
                    sum |= sum_i << shift;

                    a_word >>= 2;
                    b_word >>= 2;
                    shift += 2;
                }
                (sum, carry)
            }

        }
    };
}
impl_trit_ops_for!(u8);
impl_trit_ops_for!(u16);
impl_trit_ops_for!(u32);
impl_trit_ops_for!(u64);

