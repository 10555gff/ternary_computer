use super::logical_calculate::{Digit,DigitResult,logical_table};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP,TDIV,T3OR,T3AND,TFULLSUM,TFULLCONS};

//按字节存储、按 2-bit 逻辑处理

pub trait DibitLogic {
    fn digits_print(&self);
    fn digits_print_t(&self);
    // fn dibit_gate(&self, other: Self, table: &[[Digit; 3]; 3]) -> Self;
    // fn dibit_adder(&self, other: Self, c_in: Digit) -> (Self, Self);


    // fn dibit_tor(&self, other: Self) -> Self;
    // fn dibit_tand(&self, other: Self) -> Self;
    // fn dibit_tnor(&self, other: Self) -> Self;
    // fn dibit_tnand(&self, other: Self) -> Self;
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
        }
    };
}

impl_dibit_logic_for!(u8, 4);
impl_dibit_logic_for!(u16, 8);
impl_dibit_logic_for!(u32, 16);
impl_dibit_logic_for!(u64, 32);











    // fn dibit_adder_u32(&self, other: u32, mut carry: Digit) -> (u32, u32){
    //     let mut sum  = 0u32;
    //      for i in 0..16 {
    //         let shift = i * 2;
    //         let a = (self >> shift) & 0b11;
    //         let b = (other >> shift) & 0b11;
    //         let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as u32;
    //         carry = TFULLCONS[a as usize][b as usize][carry as usize];
    //         sum |= sum_i << shift;
    //     }
    //     (carry as u32, sum)
    // }



















// pub trait DibitLogic {
//     fn digits_print(&self);
//     fn digits_print_t(&self);

//     fn dibit_gate_u8(&self, other: u8, table: &[[Digit; 3]; 3]) -> u8;
//     //fn dibit_gate_u32(&self, other: u32, table: &[[Digit; 3]; 3]) -> u32;
//     fn dibit_adder_u8(&self, other: u8, c_in: Digit) -> (u8, u8);
//     fn dibit_adder_u32(&self, other: u32, c_in: Digit) -> (u32, u32);

//     fn dibit_tor(&self, other: u8) -> u8;
//     fn dibit_tand(&self, other: u8) -> u8;
//     fn dibit_tnor(&self, other: u8) -> u8;
//     fn dibit_tnand(&self, other: u8) -> u8;
//     fn dibit_txor(&self, other: u8) -> u8;
//     fn dibit_txnor(&self, other: u8) -> u8;
//     fn dibit_tsum(&self, other: u8) -> u8;
//     fn dibit_tcons(&self, other: u8) -> u8;
//     fn dibit_tany(&self, other: u8) -> u8;
//     fn dibit_tpoz(&self, other: u8) -> u8;
//     fn dibit_tcmp(&self, other: u8) -> u8;

//     fn dibit_gate_table3(&self, b: Self, c_in: Self, table: &[[[Digit; 3]; 3]; 3]) -> u8;
//     fn dibit_t3or(&self, b: u8, c_in: u8) -> u8;
//     fn dibit_t3and(&self, b: u8, c_in: u8) -> u8;
//     fn dibit_tfullsum(&self, b: u8, c_in: u8) -> u8;
//     fn dibit_tfullcons(&self, b: u8, c_in: u8) -> u8;
//     //fn dibit_gate_full(&self, other: &Self, c_in: Digit) -> (u8,u8);
        

    
// }









// impl DibitLogic for u8 {
//     fn dibit_gate_u8(&self, other: u8, table: &[[Digit; 3]; 3]) -> u8 {
//         let r0 = table[(self & 0b11) as usize][(other & 0b11) as usize] as u8;
//         let r1 = table[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize] as u8;
//         let r2 = table[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize] as u8;
//         let r3 = table[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize] as u8;
//         // 对整个字节进行双 bit 逻辑门，支持不同的逻辑操作
//         r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
//     }
//     fn dibit_adder_u8(&self, other: u8, c_in: Digit) -> (u8, u8) {
//         let mut carry = c_in;
//         let mut sum = 0;
//         for i in 0..4 {
//             let shift = i * 2;
//             let a = (self >> shift) & 0b11;
//             let b = (other >> shift) & 0b11;
//             let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as u8;
//             carry = TFULLCONS[a as usize][b as usize][carry as usize];
//             sum |= sum_i << shift;
//         }
//         (carry as u8, sum)
//     }
   

//     /// 使用函数别名，便于调用
//     fn dibit_tor(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TOR)
//     }
//     fn dibit_tand(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TAND)
//     }
//     fn dibit_tnor(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TNOR)
//     }
//     fn dibit_tnand(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TNAND)
//     }
//     fn dibit_txor(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TXOR)
//     }
//     fn dibit_txnor(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TXNOR)
//     }
//     fn dibit_tsum(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TSUM)
//     }
//     fn dibit_tcons(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TCONS)
//     }
//     fn dibit_tany(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TANY)
//     }
//     fn dibit_tpoz(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TPOZ)
//     }
//     fn dibit_tcmp(&self, other: u8) -> u8 {
//         self.dibit_gate_u8(other, &TCMP)
//     }





//     // fn dibit_gate_table3(&self, b: Self, c_in: Self, table: &[[[Digit; 3]; 3]; 3]) -> u8 {
//     //     // 直接使用逻辑表索引，避免闭包开销
//     //     let r0 = table[(self & 0b11) as usize][(b & 0b11) as usize][(c_in & 0b11) as usize] as u8;
//     //     let r1 = table[((self >> 2) & 0b11) as usize][((b >> 2) & 0b11) as usize][((c_in >> 2) & 0b11) as usize] as u8;
//     //     let r2 = table[((self >> 4) & 0b11) as usize][((b >> 4) & 0b11) as usize][((c_in >> 4) & 0b11) as usize] as u8;
//     //     let r3 = table[((self >> 6) & 0b11) as usize][((b >> 6) & 0b11) as usize][((c_in >> 6) & 0b11) as usize] as u8;
//     //     r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
//     // }



//     fn dibit_gate_table3(&self, b: u8, c_in: u8, table: &[[[Digit; 3]; 3]; 3]) -> u8 {
//         let mut result = 0;
//         for i in 0..4 {
//             let shift = i * 2;
//             let a = (self >> shift) & 0b11;
//             let b_val = (b >> shift) & 0b11;
//             let c = (c_in >> shift) & 0b11;
//             result |= (table[a as usize][b_val as usize][c as usize] as u8) << shift;
//         }
//         result
//     }

//     fn dibit_t3or(&self, b: u8, c_in: u8) -> u8 {
//         self.dibit_gate_table3(b, c_in,&T3OR)
//     }
//     fn dibit_t3and(&self, b: u8, c_in: u8) -> u8 {
//         self.dibit_gate_table3(b, c_in,&T3AND)
//     }

//     fn dibit_tfullsum(&self, b: u8, c_in: u8) -> u8 {
//         self.dibit_gate_table3(b, c_in,&TFULLSUM)
//     }
//     fn dibit_tfullcons(&self, b: u8, c_in: u8) -> u8 {
//         self.dibit_gate_table3(b, c_in,&TFULLCONS)
//     }










// }

























// impl DibitLogic for u32 {
//     //  fn dibit_gate_u32(&self, other: u32, table: &[[Digit; 3]; 3]) -> u32 {
//     //     let mut result = 0u32;
//     //     for i in 0..16 {
//     //         let shift = i * 2;
//     //         let a = (self >> shift) & 0b11;
//     //         let b = (other >> shift) & 0b11;
//     //         result |= (table[a as usize][b as usize] as u32) << shift;
//     //     }
//     //     result
//     // }    
    // fn dibit_adder_u32(&self, other: u32, mut carry: Digit) -> (u32, u32){
    //     let mut sum  = 0u32;
    //      for i in 0..16 {
    //         let shift = i * 2;
    //         let a = (self >> shift) & 0b11;
    //         let b = (other >> shift) & 0b11;
    //         let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as u32;
    //         carry = TFULLCONS[a as usize][b as usize][carry as usize];
    //         sum |= sum_i << shift;
    //     }
    //     (carry as u32, sum)
    // }
//     fn digits_print(&self) {
//         for i in (0..4).rev() {
//             let d = Digit::from_u8((self >> (i * 2)) & 0b11);// 提取 a 的第 i 个双 bit
//             print!("{}",d.to_char());
//         }
//         println!();
//     }
//     fn digits_print_t(&self) {
//         for i in (0..4).rev() {
//             let d = Digit::from_u8((self >> (i * 2)) & 0b11);// 提取 a 的第 i 个双 bit
//             print!("{}",d.to_char_t());
//         }
//         println!();
//     }
    
// }










