use super::logical_table::{self,Trit};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP};

#[derive(Debug, Clone, Copy)]
pub struct Trit4(pub u8); // 包装一个 u8


impl Trit4 {
    fn dibit_gate(&self, other: Trit4, table: &[[Trit; 3]; 3]) -> u8 {
        let r0 = table[(self.0 & 0b11) as usize][(other.0 & 0b11) as usize] as u8;
        let r1 = table[((self.0 >> 2) & 0b11) as usize][((other.0 >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self.0 >> 4) & 0b11) as usize][((other.0 >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self.0 >> 6) & 0b11) as usize][((other.0 >> 6) & 0b11) as usize] as u8;
        r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
    }
    /// 使用函数别名，便于调用
    pub fn dibit_tor(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TOR)
    }
    pub fn dibit_tand(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TAND)
    }
    pub fn dibit_tnor(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TNOR)
    }
    pub fn dibit_tnand(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TNAND)
    }
    pub fn dibit_txor(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TXOR)
    }
    pub fn dibit_txnor(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TXNOR)
    }
    pub fn dibit_tsum(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TSUM)
    }
    pub fn dibit_tcons(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TCONS)
    }
    pub fn dibit_tany(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TANY)
    }
    pub fn dibit_tpoz(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TPOZ)
    }
    pub fn dibit_tcmp(&self, other: Self) -> u8 {
        self.dibit_gate(other, &TCMP)
    }
}
