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

pub fn fmt(word: u8) -> String {
    let t0 = DECODE[(word & 0x03) as usize];
    let t1 = DECODE[((word >> 2) & 0x03) as usize];
    let t2 = DECODE[((word >> 4) & 0x03) as usize];
    let t3 = DECODE[((word >> 6) & 0x03) as usize];
    format!("{}{}{}{}", t3, t2, t1, t0)
}

fn read_all(word: u8) -> [u8; 4] {
    [
        word & 0x03,
        (word >> 2) & 0x03,
        (word >> 4) & 0x03,
        (word >> 6) & 0x03,
    ]
}
fn tfullsum(word: u8, word2: u8, word3: u8) -> u8 {
    let res=TFULLSUM[word as usize][word2 as usize][word3 as usize];
    res
}
fn tfullcons(word: u8, word2: u8, word3: u8) -> u8 {
    let res=TFULLCONS[word as usize][word2 as usize][word3 as usize];
    res
}

//提前算出全部进位
fn tcons3(word: u8, word2: u8, c0:u8) -> (u8, u8) {
    let a= read_all(word);
    let b= read_all(word2);

    let c1 = tfullcons(c0, a[0], b[0]);
    let c2 = tfullcons(c1, a[1], b[1]);
    let c3 = tfullcons(c2, a[2], b[2]);
    let c4 = tfullcons(c3, a[3], b[3]);

    // println!("{}{}{}{}",c4,c3,c2,c1);
    let res =c0 | (c1 << 2) | (c2 << 4) | (c3 << 6);
    (res, c4)
}
//根据PreCarry直接生成SUM结果
fn tsum3(word: u8, word2: u8, pre_carry: u8) -> u8{
    let a= read_all(word);
    let b= read_all(word2);
    let c= read_all(pre_carry);

    let s1 = tfullsum(c[0], a[0], b[0]);
    let s2 = tfullsum(c[1], a[1], b[1]);
    let s3 = tfullsum(c[2], a[2], b[2]);
    let s4 = tfullsum(c[3], a[3], b[3]);

    //println!("{}{}{}{}",s4,s3,s2,s1);
    s1 | (s2 << 2) | (s3 << 4) | (s4 << 6)
}

pub fn parall_adder(word: u8, word2: u8, carry: u8) -> (u8, u8) {
    let (pre_carry, c) = tcons3(word, word2,carry);
    let full_sum = tsum3(word, word2, pre_carry);
    (full_sum , c)
}



pub trait TritOps: Sized {
    fn read_2bit(word: Self, n: usize) -> u8;
    fn clear_2bit(word: Self, n: usize) -> Self;
    fn toggle_2bit(word: Self, n: usize) -> Self;
    fn swap_2bit(word: Self, n: usize) -> Self;
    fn set_2bit(word: Self, n: usize, value: u8) -> Self;
    fn adder(word: Self, other: Self, carry: u8) -> (Self, u8);
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


