// **加和(TSUM)逻辑表 当为TT、01、10时出1，当为11、0T、T0时出T，其余为0 此门用于半加器的加和位处理 **
pub const TSUM: [[u8; 3]; 3] = [
    [0, 1, 2],
    [1, 2, 0],
    [2, 0, 1],
];
// **共识(TCONS)逻辑表 双T出T、双1出1、其余为0 此门用于半加器的进位处理 **
pub const TCONS: [[u8; 3]; 3] = [
    [0, 0, 0],
    [0, 1, 0],
    [0, 0, 2],
];
// **调和(TANY)逻辑表 当为TT、0T、T0时出T，当为11、01、10时出1，其余为0 此门用于全加器进位处理 **
pub const TANY: [[u8; 3]; 3] = [
    [0, 1, 2],
    [1, 1, 0],
    [2, 0, 2],
];

// **非零门(TPOZ)逻辑表 当为T1、0T、T0、TT时出T，当为1T、01、10、11时出1，双0出0 此门用于检测其最高位的正负性，出T为负数，出1为正数，出0则0 **
pub const TPOZ: [[u8; 3]; 3] = [
    [0, 1, 2],
    [1, 1, 1],
    [2, 2, 2],
];
// **比较门(TCMP)逻辑表 （a=b）输出 0、 （a > b）输出 +1、(a < b)输出 -1**
pub const TCMP: [[u8; 3]; 3] = [
    [0, 2, 1],
    [1, 0, 1],
    [2, 2, 0],
];
// **除法门(TDIV)逻辑表 零不能作为除数，3属于非法值**
pub const TDIV: [[Option<u8>; 3]; 3] = [
    [None, Some(0), Some(0)],
    [None, Some(1), Some(2)],
    [None, Some(2), Some(1)],
];

// **全加器和(TFULLSUM) 逻辑表**
pub const TFULLSUM: [[[u8; 3]; 3]; 3] = [
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
// **全加器进位(TFULLCONS) 逻辑表**
pub const TFULLCONS: [[[u8; 3]; 3]; 3] = [
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


