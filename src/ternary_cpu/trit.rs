use super::logical_table::{self,Trit};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP};

#[derive(Debug, Clone, Copy)]
pub struct Trit4(pub u8); // 包装一个 u8

const MASK_EVEN: u8 = 0x55; // 偶数位掩码
const MASK_ODD:  u8 = 0xAA; // 奇数位掩码

impl Trit4 {
    // pub fn tor(&self, other: Self) -> Self {
    //     let (a,b)=(self.0,other.0);
    //     let low_bits_or = (a & MASK_EVEN) | (b & MASK_EVEN);
    //     let high_bits_and = (a & MASK_ODD) & (b & MASK_ODD);
    //     Trit4(low_bits_or | high_bits_and)
    // }
    pub fn tor(&self, other: Self) -> Self {
        let (a,b)=(self.0,other.0);
        let low_bits_or = (a | b) & MASK_EVEN;
        let high_bits_and = (a & b) & MASK_ODD;
        Trit4(low_bits_or | high_bits_and)
    }
    pub fn tand(&self, other: Self) -> Self {
        let (a,b)=(self.0,other.0);
        let low_bits_or = (a & b) & MASK_EVEN;
        let high_bits_and = (a | b) & MASK_ODD;
        Trit4(low_bits_or | high_bits_and)
    }



    // /// Case 2: 交叉逻辑 (结果位0 = A1 | B0, 结果位1 = A0 & B1)
    // pub fn tand(&self, other: Self) -> Self {
    //     let (a,b)=(self.0,other.0);
    //     let res_bit0 = (a & 0b01) & (b & 0b01);
    //     let res_bit1 = (a & 0b10) | (b & 0b10);
    //     Trit4(res_bit0 | res_bit1)
    // }












    fn dibit_gate(&self, other: Trit4, table: &[[Trit; 3]; 3]) -> Trit4 {
        let r0 = table[(self.0 & 0b11) as usize][(other.0 & 0b11) as usize] as u8;
        let r1 = table[((self.0 >> 2) & 0b11) as usize][((other.0 >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self.0 >> 4) & 0b11) as usize][((other.0 >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self.0 >> 6) & 0b11) as usize][((other.0 >> 6) & 0b11) as usize] as u8;

        Trit4(r0 | (r1 << 2) | (r2 << 4) | (r3 << 6))
    }



//     fn process_circuits(a: u8, b: u8) {
//     // 掩码定义
//     let m0 = 0x55; // 01010101 (所有偶数位/低位)
//     let m1 = 0xAA; // 10101010 (所有奇数位/高位)

//     // --- Case 1: 偶位OR, 奇位AND ---
//     let case1 = ((a & m0) | (b & m0)) | ((a & m1) & (b & m1));

//     // --- Case 2: 交叉逻辑 (Low=A1|B0, High=A0&B1) ---
//     // 思路：将 a 整体右移 1 位，让 A1 和 B0 对齐做 OR；将 a 左移 1 位，让 A0 和 B1 对齐做 AND
//     let c2_low = ((a >> 1) & m0) | (b & m0); 
//     let c2_high = ((a << 1) & m1) & (b & m1);
//     let case2 = c2_low | c2_high;

//     // --- Case 3: 位对调 (Low=A1|B1, High=A0&B0) ---
//     let c3_low = ((a & m1) | (b & m1)) >> 1;  // 高位运算后移到低位
//     let c3_high = ((a & m0) & (b & m0)) << 1; // 低位运算后移到高位
//     let case3 = c3_low | c3_high;

//     // --- Case 4: 交叉逻辑 (Low=A0&B1, High=A1|B0) ---
//     let c4_low = (a & m0) & ((b & m1) >> 1);
//     let c4_high = (a & m1) | ((b & m0) << 1);
//     let case4 = c4_low | c4_high;

//     println!("Case 1 结果: {:08b}", case1);
//     println!("Case 2 结果: {:08b}", case2);
//     println!("Case 3 结果: {:08b}", case3);
//     println!("Case 4 结果: {:08b}", case4);
// }



// /// Case 1: 偶位 (Bit 0) 执行 OR，奇位 (Bit 1) 执行 AND
// fn circuit_case_1(a: u8, b: u8) -> u8 {
//     let low_bits = (a & 0x55) | (b & 0x55);   // A0 | B0
//     let high_bits = (a & 0xAA) & (b & 0xAA);  // A1 & B1
//     low_bits | high_bits
// }

// /// Case 2: 交叉逻辑 (结果位0 = A1 | B0, 结果位1 = A0 & B1)
// fn circuit_case_2(a: u8, b: u8) -> u8 {
//     // 将 A1 右移与 B0 对齐做 OR，结果保留在位0位置
//     let res_bit0 = ((a >> 1) & 0x55) | (b & 0x55);
//     // 将 A0 左移与 B1 对齐做 AND，结果保留在位1位置
//     let res_bit1 = ((a << 1) & 0xAA) & (b & 0xAA);
//     res_bit0 | res_bit1
// }

// /// Case 3: 位对调逻辑 (结果位0 = A1 | B1, 结果位1 = A0 & B0)
// fn circuit_case_3(a: u8, b: u8) -> u8 {
//     // 高位对(A1, B1)做 OR，然后右移到低位(0)
//     let res_bit0 = ((a & 0xAA) | (b & 0xAA)) >> 1;
//     // 低位对(A0, B0)做 AND，然后左移到高位(1)
//     let res_bit1 = ((a & 0x55) & (b & 0x55)) << 1;
//     res_bit0 | res_bit1
// }

// /// Case 4: 交叉逻辑 (结果位0 = A0 & B1, 结果位1 = A1 | B0)
// fn circuit_case_4(a: u8, b: u8) -> u8 {
//     // 将 B1 右移与 A0 对齐做 AND，结果保留在位0位置
//     let res_bit0 = (a & 0x55) & ((b >> 1) & 0x55);
//     // 将 B0 左移与 A1 对齐做 OR，结果保留在位1位置
//     let res_bit1 = (a & 0xAA) | ((b << 1) & 0xAA);
//     res_bit0 | res_bit1
// }

// fn main() {
//     let a: u8 = 0b1100_1010;
//     let b: u8 = 0b1010_0101;

//     println!("Input A:  {:08b}", a);
//     println!("Input B:  {:08b}", b);
//     println!("--------------------");
//     println!("Case 1:   {:08b}", circuit_case_1(a, b));
//     println!("Case 2:   {:08b}", circuit_case_2(a, b));
//     println!("Case 3:   {:08b}", circuit_case_3(a, b));
//     println!("Case 4:   {:08b}", circuit_case_4(a, b));
// }


    /// 使用函数别名，便于调用
    pub fn dibit_tor(&self, other: Self) -> Self {
        self.dibit_gate(other, &TOR)
    }
    pub fn dibit_tand(&self, other: Self) -> Self {
        self.dibit_gate(other, &TAND)
    }
    pub fn dibit_tnor(&self, other: Self) -> Self {
        self.dibit_gate(other, &TNOR)
    }
    pub fn dibit_tnand(&self, other: Self) -> Self {
        self.dibit_gate(other, &TNAND)
    }
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
