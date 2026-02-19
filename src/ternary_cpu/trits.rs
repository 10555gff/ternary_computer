use std::fmt;
use super::logical_table::{TSUM,TANY,TPOZ,TCMP,TFULLSUM,TFULLCONS};

// 定义位掩码常量，增加可读性
const MASK_EVEN: u8 = 0x55; // 01010101b (c0 位)
const MASK_ODD:  u8 = 0xAA; // 10101010b (c1 位)
const SHIFT: [u8;4] = [0,2,4,6];
const MASK:  [u8;4] = [0x03,0x0C,0x30,0xC0];
const DECODE: [char;4]=['0','1','T','X'];

fn set_2bit(word: u8, k: usize, value: u8) -> u8 {
    let mask  = MASK[k];
    let shift = SHIFT[k];
    (word & !mask) | ((value & 0x03) << shift)
}
fn read_2bit(word:u8,k:usize)->u8{
    (word & MASK[k]) >> SHIFT[k]
}
fn clear_2bit(word: u8, k: usize) -> u8 {
    word & !MASK[k]
}
fn toggle_2bit(word: u8, k: usize) -> u8 {
    word ^ MASK[k]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trit4(pub u8); // 包装一个 u8

impl Trit4 {
    pub fn get(&self, i:usize)->u8 { read_2bit(self.0,i) }
    pub fn clear(&self, i:usize)->u8 { clear_2bit(self.0,i) }
    pub fn toggle(&self, i:usize)->u8 { toggle_2bit(self.0,i) }
    pub fn set(&mut self,i:usize,v:u8){
        self.0 = set_2bit(self.0,i,v)
    }

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
            4=>res=((or & (or >> 1)) & MASK_EVEN) | ((and | (and << 1)) & MASK_ODD),//txor
            5=>res=((and | (and >> 1)) & MASK_EVEN) | ((or & (or << 1)) & MASK_ODD),//tnxor
            6=>res=and,//tcons
            7=>res=((and >> 1) & MASK_EVEN) | ((and << 1) & MASK_ODD),//tncons
            8 => {
                let m = (or & (or >> 1)) & MASK_EVEN;
                res = or & !(m | (m << 1));
            },//tany
            9 => {
                let nor = !or;
                let m = (nor & (nor >> 1)) & MASK_EVEN;
                res = nor & !(m | (m << 1));
            },//tnany
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
        let (a, b) = (self.0, other.0);
        let res=((a | b) & 0x55) | ((a & b) & 0xAA);
        Trit4(res)
    }
    pub fn tand(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        let res=((a & b) & 0x55) | ((a | b) & 0xAA);
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
        let res=((and | (and << 1)) & 0xAA) | ((or & (or >> 1)) & 0x55);
        Trit4(res)
    }
    pub fn tnxor(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let res=((or & (or << 1)) & 0xAA) | ((and | (and >> 1)) & 0x55);
        Trit4(res)
    }

    pub fn tcons(self, other: Self) -> Self {
        Trit4(self.0 & other.0)
    }
    pub fn tncons(self, other: Self) -> Self {
        let val = self.0 & other.0;
        let res=((val >> 1) & 0x55) | ((val << 1) & 0xAA);
        Trit4(res)
    }

    pub fn tany(self, other: Self) -> Self {
        let or:u8 =self.0 | other.0;
        let mask = (or & (or >> 1)) & 0x55;
        let res = or & !(mask | (mask << 1));
        Trit4(res)
    }
    pub fn tnany(self, other: Self) -> Self {
        let nor:u8 = !(self.0 | other.0);
        let mask = (nor & (nor >> 1)) & 0x55;
        let res = nor & !(mask | (mask << 1));
        Trit4(res)
    }

    pub fn tsum(self, other: Self) -> Self {
        let (or, and) = self.or_and(other);
        let r1 = ((and >> 1) & 0x55) | ((and << 1) & 0xAA);
        let r2 = or;//tany


        let or =r1 | r2;
   
        let mask = (or & (or >> 1)) & 0x55;
        let r3 = or & !(mask | (mask << 1));




        let or = r1 | r3;
        // let mask = (or & (or >> 1)) & 0x55;
        let res = or ;




        // let r1=self.tncons(other);
        // let r2=self.tany(other);

        // let r3=r1.tany(r2);
        // let r4=r3.tany(r1);

        Trit4(res)
    }


    fn dibit_gate(&self, other: Trit4, table: &[[u8; 3]; 3]) -> Trit4 {
        let r0 = table[(self.0 & 0b11) as usize][(other.0 & 0b11) as usize] as u8;
        let r1 = table[((self.0 >> 2) & 0b11) as usize][((other.0 >> 2) & 0b11) as usize] as u8;
        let r2 = table[((self.0 >> 4) & 0b11) as usize][((other.0 >> 4) & 0b11) as usize] as u8;
        let r3 = table[((self.0 >> 6) & 0b11) as usize][((other.0 >> 6) & 0b11) as usize] as u8;

        Trit4(r0 | (r1 << 2) | (r2 << 4) | (r3 << 6))
    }

    pub fn dibit_adder(&self, other: Trit4, mut carry: u8) -> (u8, Trit4) {
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
    pub fn dibit_tsum(&self, other: Self) -> Self {
        self.dibit_gate(other, &TSUM)
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


impl fmt::Display for Trit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 提取 4 个 trit
        let t0 = DECODE[self.get(0) as usize];
        let t1 = DECODE[self.get(1) as usize];
        let t2 = DECODE[self.get(2) as usize];
        let t3 = DECODE[self.get(3) as usize];
        
        // 格式化为 [t3, t2, t1, t0] (高位在前符合数值习惯)
        write!(f, "Trit4[{}, {}, {}, {}]", t3, t2, t1, t0)
    }
}
