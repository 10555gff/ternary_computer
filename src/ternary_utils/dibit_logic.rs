use super::logical_calculate::{Digit,DigitResult,logical_table};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP,TDIV,T3OR,T3AND,TFULLSUM,TFULLCONS};

//按字节存储、按 2-bit 逻辑处理
pub trait DibitLogic: Sized{
    fn digits_print(&self);
    fn digits_print_t(&self);
    fn dibit_adder(&self, other: Self, c_in: Digit) -> (Self, Self);


    fn dibit_gate(&self, other: Self, table: &[[Digit; 3]; 3]) -> Self;
    fn dibit_tor(&self, other: Self) -> Self;
    fn dibit_tand(&self, other: Self) -> Self;
    fn dibit_tnor(&self, other: Self) -> Self;
    fn dibit_tnand(&self, other: Self) -> Self;
    // fn dibit_txor(&self, other: Self) -> Self;
    // fn dibit_txnor(&self, other: Self) -> Self;
    // fn dibit_tsum(&self, other: Self) -> Self;
    // fn dibit_tcons(&self, other: Self) -> Self;
    // fn dibit_tany(&self, other: Self) -> Self;
    // fn dibit_tpoz(&self, other: Self) -> Self;
    // fn dibit_tcmp(&self, other: Self) -> Self;
    // fn dibit_gate_table3(&self, b: Self, c_in: Self, table: &[[[Digit; 3]; 3]; 3]) -> Self;
    // fn dibit_t3or(&self, b: Self, c_in: Self) -> Self;
    // fn dibit_t3and(&self, b: Self, c_in: Self) -> Self;
    // fn dibit_tfullsum(&self, b: Self, c_in: Self) -> Self;
    // fn dibit_tfullcons(&self, b: Self, c_in: Self) -> Self;
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
            fn dibit_adder(&self, other: $t, mut carry: Digit) -> ($t, $t) {
                let mut sum = 0;
                for i in 0..$count {
                    let shift = i * 2;
                    let a = (self >> shift) & 0b11;
                    let b = (other >> shift) & 0b11;
                    let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as $t;
                    carry = TFULLCONS[a as usize][b as usize][carry as usize];
                    sum |= sum_i << shift;
                }
                (carry as $t, sum)
            }

             #[inline(always)]
            fn dibit_gate(&self, other:$t, table: &[[Digit; 3]; 3]) -> $t {
                let mut result=0;
                for i in 0..$count {
                    let shift = i * 2;
                    let da = (self >> shift) & 0b11;
                    let db = (other >> shift) & 0b11;
                    result |= (table[da as usize][db as usize] as $t) << shift;
                }
                result
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

        }
    };
}


impl_dibit_logic_for!(u8, 4);
impl_dibit_logic_for!(u16, 8);
impl_dibit_logic_for!(u32, 16);
impl_dibit_logic_for!(u64, 32);

