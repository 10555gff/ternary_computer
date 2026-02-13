use super::logical_table::{self,Trit};
use logical_table::{TOR,TAND,TNOR,TNAND,TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP};

pub trait DibitOperations {

    fn dibit_gate(&self, other: Self, table: &[[Trit; 3]; 3]) -> Self;
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

impl DibitOperations for u8 {
    fn dibit_gate(&self, other: u8, table: &[[Trit; 3]; 3]) -> u8 {
        let r0 = table[(self & 0b11) as usize][(other & 0b11) as usize] as u8;
        let r1 = table[((self >> 2) & 0b11) as usize][((other >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self >> 4) & 0b11) as usize][((other >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self >> 6) & 0b11) as usize][((other >> 6) & 0b11) as usize] as u8;
        r0 | (r1 << 2) | (r2 << 4) | (r3 << 6)
    }

    /// 使用函数别名，便于调用
    fn dibit_tor(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TOR)
    }
    fn dibit_tand(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TAND)
    }
    fn dibit_tnor(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TNOR)
    }
    fn dibit_tnand(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TNAND)
    }
    fn dibit_txor(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TXOR)
    }
    fn dibit_txnor(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TXNOR)
    }
    fn dibit_tsum(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TSUM)
    }
    fn dibit_tcons(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TCONS)
    }
    fn dibit_tany(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TANY)
    }
    fn dibit_tpoz(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TPOZ)
    }
    fn dibit_tcmp(&self, other: u8) -> u8 {
        self.dibit_gate(other, &TCMP)
    }
}