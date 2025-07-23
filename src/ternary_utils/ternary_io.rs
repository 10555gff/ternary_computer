use super::logical_calculate::Digit;
use core::ops::{Deref, DerefMut, Neg, Add, BitAnd, BitOr, BitXor, Div, Mul, Sub, Shl, Shr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ternary(pub Vec<Digit>);

impl Deref for Ternary {
    type Target = Vec<Digit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Ternary {
    //不需要再次声明type Target ，因为它已经从 Deref 那里继承了
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    pub fn to_string(&self) -> String {
        self.iter().map(Digit::to_char).collect()
    }

    pub fn to_sign(&self) -> Digit {
        let mut state=Digit::Z;
        for &digit in self.iter() {
            state=state.tpoz(digit);
        }
        state
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
        let (stack1,stack2)=(self.pad_left(max_len),other.pad_left(max_len));

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


    pub fn adder_stack(mut self, mut other: Self, mut c_in: Digit) -> Self {
        let mut result:Vec<Digit> = Vec::new();//存储和
        
        //Rust标准库中Vec,天然支持后进先出（LIFO），用栈协同弹出，倒序遍历, 支持不同长度
        while !self.is_empty() || !other.is_empty() {
            let v1 = self.pop().unwrap_or(Digit::Z);
            let v2 = other.pop().unwrap_or(Digit::Z);
    
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


impl Neg for &Ternary {//不消耗原值，借用
    type Output = Ternary;

    fn neg(self) -> Self::Output {
        self.to_neg()
    }
}
impl Neg for Ternary {//消耗原值，借用自身
    type Output = Ternary;

    fn neg(self) -> Self::Output {
        (&self).to_neg()
    }
}

impl Add<&Ternary> for &Ternary {//&a + &b，不消耗原值，适合重用,借用 和 借用
    type Output = Ternary;

    fn add(self, rhs: &Ternary) -> Self::Output {
        self.adder_base(rhs, Digit::Z)
    }
}
impl Add<Ternary> for Ternary {//a + b，消耗原值，优化内存操作,所有权 和 所有权
    type Output = Ternary;

    fn add(self, rhs: Ternary) -> Self::Output {
        self.adder_stack(rhs, Digit::Z)
    }
}
impl Add<Digit> for &Ternary {//&a + b，Ternary+Digit
    type Output = Ternary;

    fn add(self, rhs: Digit) -> Self::Output {
        let other=Ternary::new(vec![0]);
        self.adder_base(&other, rhs)
    }
}



impl Sub<&Ternary> for &Ternary {//&a - &b，不消耗原值，适合重用,借用 和 借用
    type Output = Ternary;

    fn sub(self, rhs: &Ternary) -> Self::Output {
        let other=-rhs;//获取相反数
        self.adder_base(&other, Digit::Z)
    }
}
impl Sub<Ternary> for Ternary {//a - b，消耗原值，优化内存操作,所有权 和 所有权
    type Output = Ternary;

    fn sub(self, rhs: Ternary) -> Self::Output {
        let other=-rhs;//获取相反数
        self.adder_stack(other, Digit::Z)
    }
}
impl Sub<Digit> for &Ternary {//&a - b，Ternary-Digit
    type Output = Ternary;

    fn sub(self, rhs: Digit) -> Self::Output {
        let other=Ternary::new(vec![0]);
        self.adder_base(&other, rhs.to_neg())
    }
}




