use super::logical_table::{self,Trit};
use logical_table::{TXOR,TXNOR,TSUM,TCONS,TANY,TPOZ,TCMP};

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
            0=>res=(or & 0x55) | (and & 0xAA),//tor
            1=>res=(and & 0x55) | (or & 0xAA),//tand
            2=>res=((or & 0x55) << 1) | ((and & 0xAA) >> 1),//tnor
            3=>res=((and & 0x55) << 1) | ((or & 0xAA) >> 1),//tnand
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

    fn dibit_gate(&self, other: Trit4, table: &[[Trit; 3]; 3]) -> Trit4 {
        let r0 = table[(self.0 & 0b11) as usize][(other.0 & 0b11) as usize] as u8;
        let r1 = table[((self.0 >> 2) & 0b11) as usize][((other.0 >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self.0 >> 4) & 0b11) as usize][((other.0 >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self.0 >> 6) & 0b11) as usize][((other.0 >> 6) & 0b11) as usize] as u8;

        Trit4(r0 | (r1 << 2) | (r2 << 4) | (r3 << 6))
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
