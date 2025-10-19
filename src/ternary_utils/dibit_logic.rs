use super::logical_calculate::{Digit,logical_table};
use logical_table::{TNEG,TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP,TFULLSUM,TFULLCONS};

pub trait DibitLogic: Sized{//按字节存储、按 2-bit 逻辑处理
    fn digits_print(&self);
    fn digits_print_t(&self);
    fn dibit_adder(&self, other: Self, carry: Digit) -> (Digit, Self);

    fn dibit_neg(&self) -> Self;
    fn dibit_gate(&self, other: Self, table: &[[Digit; 3]; 3]) -> Self;
    fn dibit_tor(&self, other: Self) -> Self;
    fn dibit_tand(&self, other: Self) -> Self;
    fn dibit_tnor(&self, other: Self) -> Self;
    fn dibit_tnand(&self, other: Self) -> Self;
    fn dibit_txor(&self, other: Self) -> Self;
    fn dibit_txnor(&self, other: Self) -> Self;
    fn dibit_tsum(&self, other: Self) -> Self;
    fn dibit_tcons(&self, other: Self) -> Self;
    fn dibit_tany(&self, other: Self) -> Self;
    fn dibit_tpoz(&self, other: Self) -> Self;
    fn dibit_tcmp(&self, other: Self) -> Self;
}


macro_rules! impl_dibit_logic_for {
    ($t:ty, $count:expr) => {
        impl DibitLogic for $t {
            fn digits_print(&self) {
                for i in (0..$count).rev() {
                    let d = Digit::from_u8(((self >> (i * 2)) & 0b11) as u8);
                    print!("{}", d.to_char());
                }
                println!();
            }

            fn digits_print_t(&self) {
                for i in (0..$count).rev() {
                    let d = Digit::from_u8(((self >> (i * 2)) & 0b11) as u8);
                    print!("{}", d.to_char_t());
                }
                println!();
            }

            #[inline(always)]
            fn dibit_adder(&self, other: $t, mut carry: Digit) -> (Digit, $t) {
                let mut sum = 0;
                for i in 0..$count {
                    let shift = i * 2;
                    let a = (self >> shift) & 0b11;
                    let b = (other >> shift) & 0b11;
                    let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as $t;
                    carry = TFULLCONS[a as usize][b as usize][carry as usize];
                    sum |= sum_i << shift;
                }
                (carry , sum)
            }


            #[inline(always)]
            fn dibit_neg(&self) -> Self {
                let mut result: $t = 0;
                for i in 0..$count {
                    let shift = i * 2;
                    let d = (self >> shift) & 0b11;
                    result |= (TNEG[d as usize] as $t) << shift;
                }
                result
            }

        
            #[inline(always)]
            fn dibit_gate(&self, other: $t, table: &[[Digit; 3]; 3]) -> $t {
                if $count == 4 {// u8 展开
                    let r0 = table[(self & 0b11) as usize][(other & 0b11) as usize] as $t;
                    let r1 = table[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize] as $t;
                    let r2 = table[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize] as $t;
                    let r3 = table[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize] as $t;
                    r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
                } else {// 为 u16、u32、u64 使用循环实现
                    let mut result: $t = 0;
                    for i in 0..$count {
                        let shift = i * 2;
                        let da = (self >> shift) & 0b11;
                        let db = (other >> shift) & 0b11;
                        result |= (table[da as usize][db as usize] as $t) << shift;
                    }
                    result
                }
            }
            /// 使用函数别名，便于调用
            fn dibit_tor(&self, other: $t) -> $t {
                self.dibit_gate(other, &TOR)
            }
            fn dibit_tand(&self, other: $t) -> $t {
                self.dibit_gate(other, &TAND)
            }
            fn dibit_tnor(&self, other: $t) -> $t{
                self.dibit_gate(other, &TNOR)
            }
            fn dibit_tnand(&self, other: $t) -> $t{
                self.dibit_gate(other, &TNAND)
            }
            fn dibit_txor(&self, other: $t) -> $t {
                self.dibit_gate(other, &TXOR)
            }
            fn dibit_txnor(&self, other: $t) -> $t {
                self.dibit_gate(other, &TXNOR)
            }
            fn dibit_tsum(&self, other: $t) -> $t {
                self.dibit_gate(other, &TSUM)
            }
            fn dibit_tcons(&self, other: $t) -> $t {
                self.dibit_gate(other, &TCONS)
            }
            fn dibit_tany(&self, other: $t) -> $t {
                self.dibit_gate(other, &TANY)
            }
            fn dibit_tpoz(&self, other: $t) -> $t {
                self.dibit_gate(other, &TPOZ)
            }
            fn dibit_tcmp(&self, other: $t) -> $t {
                self.dibit_gate(other, &TCMP)
            }
        }
    };
}


impl_dibit_logic_for!(u8, 4);
impl_dibit_logic_for!(u16, 8);
impl_dibit_logic_for!(u32, 16);
impl_dibit_logic_for!(u64, 32);

