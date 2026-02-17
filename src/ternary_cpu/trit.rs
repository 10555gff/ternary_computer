use super::logical_table::{self,Trit};
use logical_table::{TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP,TFULLSUM,TFULLCONS};

// 定义位掩码常量，增加可读性
const MASK_EVEN: u8 = 0x55; // 01010101b (c0 位)
const MASK_ODD:  u8 = 0xAA; // 10101010b (c1 位)

#[derive(Debug, Clone, Copy)]
pub struct Trit4(pub u8); // 包装一个 u8

impl Trit4 {
    #[inline(always)]
    fn or_and(self, other: Self) -> (u8, u8) {
        let or  = self.0 | other.0;
        let and = self.0 & other.0;
        (or, and)
    }

    #[inline(always)]
    pub fn gate_core(&self, other: Self, code:u8)-> Self{
        let (or, and) = self.or_and(other);
        let mut res:u8=0;

        match code{
            0=>res=(or & MASK_EVEN) | (and & MASK_ODD),//tor
            1=>res=(and & MASK_EVEN) | (or & MASK_ODD),//tand
            2=>res=((or & MASK_EVEN) << 1) | ((and & MASK_ODD) >> 1),//tnor
            3=>res=((and & MASK_EVEN) << 1) | ((or & MASK_ODD) >> 1),//tnand
            _ =>println!("undefine"),
        }
        Trit4(res)
    }

    pub fn tneg(&self) -> Self {
        let val = self.0;
        let res=((val & 0xAA) >> 1) | ((val & 0x55) << 1);
        Trit4(res)
    }

    pub fn tor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=(or & 0x55) | (and & 0xAA);
        Trit4(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=(and & 0x55) | (or & 0xAA);
        Trit4(res)
    }

    pub fn tnor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((and & 0xAA) >> 1) | ((or & 0x55) << 1);
        Trit4(res)
    }
    pub fn tnand(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((or & 0xAA) >> 1) | ((and & 0x55) << 1);
        Trit4(res)
    }

    pub fn txor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);

        // 1. 提取 or 的高低位交集 (原本 txor 的高位 -> 现在的低位)
        // (or & 0x55) & (or >> 1) 得到两位置 1 的情况
        let res_low = (or & (or >> 1)) & 0x55;

        // 2. 提取 and 的高低位并集 (原本 txor 的低位 -> 现在的高位)
        // (and | (and << 1)) & 0xAA 得到任一位置 1 的情况
        let res_high = (and | (and << 1)) & 0xAA;
        //println!("ffffffffffffffffffffffffbbffffff");

        Trit4(res_low | res_high)
    }


    pub fn tnxor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);

        let res_low = (or & (or << 1)) & 0xAA;

        // 2. 提取 and 的高低位并集 (原本 txor 的低位 -> 现在的高位)
        // (and | (and << 1)) & 0xAA 得到任一位置 1 的情况
        let res_high = (and | (and >> 1)) & 0x55;

        
        println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        Trit4(res_low | res_high)
    }







    // pub fn (self, other: Self) -> Self {
    //     let (or, and) = self.or_and(other);

    // }

    // pub fn tnxor(self, other: Self) -> Self {
    //     let (or, and) = self.or_and(other);

    //     let low_or = or & 0x55;
    //     let high_or = (or & 0xAA) >> 1;
    //     let low_and = and & 0x55;
    //     let high_and = (and & 0xAA) >> 1;
    //     let res = ((low_or & high_or) << 1) | (low_and | high_and);
    //     Trit4(res)
    // }




// pub fn tnxor(self, other: Self) -> Self {
//     let (or, and) = self.or_and(other);

//     let low_or  = or  & 0x55;
//     let high_or = (or  & 0xAA) >> 1;
//     let low_and = and & 0x55;
//     let high_and= (and & 0xAA) >> 1;

//     // 先算 txor 的结果
//     let xor_res = ((low_or & high_or) << 1) | (low_and | high_and);

//     // 直接在 xor_res 上做 tneg（高低位互换）
//     let txnor_res = ((xor_res & 0xAA) >> 1) | ((xor_res & 0x55) << 1);

//     Trit4(txnor_res)
// }


    fn dibit_gate(&self, other: Trit4, table: &[[Trit; 3]; 3]) -> Trit4 {
        let r0 = table[(self.0 & 0b11) as usize][(other.0 & 0b11) as usize] as u8;
        let r1 = table[((self.0 >> 2) & 0b11) as usize][((other.0 >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self.0 >> 4) & 0b11) as usize][((other.0 >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self.0 >> 6) & 0b11) as usize][((other.0 >> 6) & 0b11) as usize] as u8;

        Trit4(r0 | (r1 << 2) | (r2 << 4) | (r3 << 6))
    }

    pub fn dibit_adder(&self, other: Trit4, mut carry: Trit) -> (Trit, Trit4) {
        let mut sum = 0;
        for i in 0..4 {
            let shift = i * 2;
            let a = (self.0 >> shift) & 0b11;
            let b = (other.0 >> shift) & 0b11;
            let sum_i = TFULLSUM[a as usize][b as usize][carry as usize] as u8;
            carry = TFULLCONS[a as usize][b as usize][carry as usize];
            sum |= sum_i << shift;
        }
        (carry , Trit4(sum))
    }

    /// 使用函数别名，便于调用
    pub fn dibit_txor(&self, other: Self) -> Self {
        self.dibit_gate(other, &TXOR)
    }
    pub fn dibit_txnor(&self, other: Self) -> Self {
        self.dibit_gate(other, &TXNOR)
    }
    pub fn dibit_tsum(&self, other: Self) -> Self {
        self.dibit_gate(other, &TSUM)
    }
    pub fn dibit_tcons(&self, other: Self) -> Self {
        self.dibit_gate(other, &TCONS)
    }
    pub fn dibit_tany(&self, other: Self) -> Self {
        self.dibit_gate(other, &TANY)
    }
    pub fn dibit_tpoz(&self, other: Self) -> Self {
        self.dibit_gate(other, &TPOZ)
    }
    pub fn dibit_tcmp(&self, other: Self) -> Self {
        self.dibit_gate(other, &TCMP)
    }
}
