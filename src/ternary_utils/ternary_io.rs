use super::logical_calculate::{logical_table,Digit};

use core::ops::{Deref,Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub, Shl, Shr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ternary(pub Vec<Digit>);

impl Deref for Ternary {
    type Target = Vec<Digit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ternary {
    pub fn new(vec: Vec<u8>) -> Self {
        Self(vec.into_iter().map(Digit::from_u8).collect())
    }

    pub fn parse(s: &str) -> Self {
        Self(s.chars().map(Digit::from_char).collect())
    }

    pub fn to_neg(&self) -> Self {
        Self(self.iter().copied().map(Digit::to_neg).collect())
    }

    pub fn to_sign(&self) -> Digit {
        let mut state=Digit::Z;
        for &digit in self.iter() {
            state=state.tpoz(digit);
            
            
            //= logical_table::TPOZ[state as usize][digit as usize];
        }
        state
    }

    pub fn to_string(&self) -> String {
        self.iter().map(Digit::to_char).collect()
    }

    /// Converts the `Ternary` object to its integer (decimal) representation.
    pub fn to_dec(&self) -> i64 {
        let mut dec = 0;
        for (rank, digit) in self.iter().rev().enumerate() {
            dec += digit.to_i8() as i64 * 3_i64.pow(rank as u32);
        }
        dec
    }

    /// Creates a balanced ternary number from a decimal integer.
    pub fn from_dec(mut dec: i64) -> Self {
        if dec == 0 {return Self(vec![Digit::Z]);}

        let mut repr = Vec::new();
        while dec!=0 {
            let remainder = dec % 3;
            match remainder {
                -1 | 2=>{
                    repr.push(Digit::N);
                    dec=(dec+1)/3;
                },
                -2 | 1=>{
                    repr.push(Digit::P);
                    dec=(dec-1)/3;
                },
                _=>{
                    repr.push(Digit::Z);
                    dec=dec/3;
                },
            }
        }
        repr.reverse();
        Self(repr)
    }

    pub fn digits_print(&self) {
        for d in self.iter() {
            print!("{}", d.to_char());
        }
        println!();
    }
    pub fn digits_print_t(&self) {
        for d in self.iter() {
            print!("{}", d.to_char_t());
        }
        println!();
    }


    fn pad_left(&self, max_len: usize) -> Vec<Digit> {
        let mut result = Vec::with_capacity(max_len);
        let pad_len = max_len.saturating_sub(self.len());
        result.extend(std::iter::repeat(Digit::Z).take(pad_len));
        result.extend_from_slice(self);
        result
    }

    pub fn adder_base(&self, other: &Self, mut c_in: Digit)-> Self{
        let mut result:Vec<Digit> = Vec::new();//存储和

        let max_len=self.len().max(other.len());
        let stack1=self.pad_left(max_len);
        let stack2=other.pad_left(max_len);

        for i in (0..max_len).rev(){
            let (v1,v2)=(stack1[i],stack2[i]);
            let fulladder_out=v1.full_adder(v2, c_in);
            result.push(fulladder_out.sum);//存结果
            c_in=fulladder_out.carry;//进位传递
        }
        if c_in!=Digit::Z{
            result.push(c_in);
        }

        result.reverse(); // 反转，从高位到低位排列
        Self(result)
    }

}
















impl Add<&Ternary> for &Ternary {
    type Output = Ternary;
    fn add(self, rhs: &Ternary) -> Self::Output {
        self.adder_base(rhs, Digit::Z)
    }
}

impl Add<Digit> for &Ternary {
    type Output = Ternary;

    fn add(self, rhs: Digit) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_add(rhs.to_i8() as i64)
                .expect("Overflow in addition."),
        )
    }
}




