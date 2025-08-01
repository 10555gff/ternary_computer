pub mod ternary_utils;
pub use ternary_utils::ternary_io;
pub use ternary_utils::logical_calculate;

/// A `Ternary` object in this module represents a number in the balanced ternary numeral system.
/// Balanced ternary is a non-standard positional numeral system that uses three digits: {-1, 0, +1}
/// - **`Digit` Enum**:
/// - `N` for -1
/// - `Z` for 0
/// - `P` for +1
///
/// # Examples
///
/// ```
/// use ternary_arithmetic::logical_calculate::Digit;
/// use ternary_arithmetic::ternary_io::Ternary;
/// 
/// let t1=Ternary::parse("+-+++++++++++++-+");
/// let t2=Ternary::parse("++-+-");
/// let t3=&t1 / &t2;
/// t3.quotient.digits_print();
/// 
/// ```



#[cfg(test)]
mod tests {
    use super::ternary_io::Ternary;
    use super::logical_calculate::Digit;

    #[test]
    fn test_digit_base() {
        let a=Digit::N;//+0-
        let result=a.to_char();
        let result2=Digit::from_char(result);
        assert_eq!(result,'-');
        assert_eq!(result2,Digit::N);

        let a=Digit::N;//T01
        let result=a.to_char_t();
        let result2=Digit::from_char_t(result);
        assert_eq!(result,'T');
        assert_eq!(result2,Digit::N);

        let a=Digit::N;//012
        let result=a.to_u8();
        let result2=Digit::from_u8(result);
        assert_eq!(result,2);
        assert_eq!(result2,Digit::N);

        let a=Digit::N;//-101
        let result=a.to_i8();
        let result2=Digit::from_i8(result);
        assert_eq!(result,-1);
        assert_eq!(result2,Digit::N);

        let a=Digit::N;//neg门
        let result=a.to_neg();
        assert_eq!(result,Digit::P);


        let a=Digit::N;
        let result=a.post();//右偏门，下一位
        let result2=result.pre();//左偏门，上一位
        assert_eq!(result2,Digit::N);

        let a=Digit::N;
        let result=a.max();//最大门max(a,0)
        let result2=a.min();//最小门min(a,0)
        assert_eq!(result,Digit::Z);
        assert_eq!(result2,Digit::N);
    }

    #[test]
    fn test_digit_logic() {
        let (a,b)=(Digit::P,Digit::N);
        let r1=a.tor(b);
        let r2=a.tand(b);
        let r3=a.tnor(b);
        let r4=a.tnand(b);
        let b1=a.tnor(!b);
        let b2 = !r1;//平衡三进制摩根定律定律应用
        assert_eq!(r2, b1);
        assert_eq!(r4, r1);
        assert_eq!(b2, r3);

        let (a,b)=(Digit::P,Digit::N);
        // let result=a.txor(b);//异或门
        // let result=a.txnor(b);//同或门

        // let result=a.tsum(b);//加和门，半加器sum位
        // let result=a.tcons(b);//共识门，半加器carry位
        // let result=a.tany(b);//调和门，两个半加器的carry位

        // let result=a.tpoz(b);//非零门,高位0判断
        // let result=a.tcmp(b);//对比门,大小判断

        // let result=a.tdiv(b);//除法门
        // let result=a.t3and(b,Digit::N);//三与门
        let result=a.t3or(b,Digit::N);//三或门
        assert_eq!(result,Digit::P);

        let (a,b)=(Digit::N,Digit::N);
        let result = a.half_adder(b);//半加器
        assert_eq!((result.carry,result.sum), (Digit::N,Digit::P));

        let (a,b,c_in)=(Digit::P,Digit::P,Digit::P);
        let result = a.full_adder(b, c_in);//全加器基于三维数组
        assert_eq!((result.carry,result.sum), (Digit::P,Digit::Z));

        let (a,b,c_in)=(Digit::Z,Digit::P,Digit::P);
        let result = a.full_adder_gate(b, c_in);//全加器基于逻辑门
        assert_eq!((result.carry,result.sum), (Digit::P,Digit::N));
    }

    #[test]
    fn test_digit_ops() {
        let x=Digit::P;
        let result=-x;
        let result2=!result;
        assert_eq!(result, Digit::N);//数学的负号
        assert_eq!(result2, Digit::P);//逻辑的非

        let (a,b)=(Digit::P,Digit::N);
        let result = a & b;//与门
        assert_eq!(result, Digit::N);

        let (a,b)=(Digit::P,Digit::N);
        let result = a | b;//或门
        assert_eq!(result, Digit::P);

        let (a,b)=(Digit::P,Digit::Z);
        let result = a ^ b;//异或门
        assert_eq!(result, Digit::Z);

        let (a,b)=(Digit::P,Digit::N);
        let result = a * b;//乘法
        assert_eq!(result, Digit::N);

        let (a,b)=(Digit::P,Digit::P);
        let result = a / b;//除法
        assert_eq!(result, Digit::P);

        let (a,b)=(Digit::P,Digit::P);
        let result = a + b;//加法
        assert_eq!((result.carry,result.sum), (Digit::P,Digit::N));

        let (a,b)=(Digit::P,Digit::N);
        let result = a - b;//减法
        assert_eq!((result.carry,result.sum), (Digit::P,Digit::N));
    }
//---------------------------------------------------------------------------------------------------------------
    #[test]
    fn test_convert_to_ternary() {
        let input = Ternary::parse("+0-");//字符解析
        let input2 = Ternary::parse_t("T01");
        input2.digits_print();//打印+0-
        input2.digits_print_t();//打印T01

        let expected = Ternary::new(vec![1,0,2]);//新建Ternary
        let expected2 = Ternary::new_d(vec![Digit::P,Digit::Z,Digit::N]);
        assert_eq!(input, expected);
        assert_eq!(input, expected2);

        let result=input.to_neg();//获取相反值
        let result2=result.to_string();//Ternary转成String
        assert_eq!(result, input2);
        assert_eq!(result2, "-0+");

        
        let input = Ternary::parse("00-0++0-+0-");
        let result=input.to_sign();//获取符号
        assert_eq!(result, Digit::N);

        let input = Ternary::parse("+++");
        let result=input.to_dec();//转成数值
        assert_eq!(result, 13);

        let input = Ternary::parse("++++");
        let result=Ternary::from_dec(40);
        assert_eq!(result, input);
        println!("{:?}",result);


        let a = Ternary::parse("+0-");
        let b = Ternary::parse("++0++");
        let result=a.tor(&b);
        result.digits_print_t();

        let input = Ternary::parse("000-0-+00++");
        let result=input.trim_zeros();//转成数值
        println!("{:?}",result);
    }


    #[test]
    fn test_ternary_ops() {
        let repr_neg120 = - &Ternary::parse("++++0");//获取相反数
        assert_eq!(repr_neg120.to_dec(), -120);
        assert_eq!(repr_neg120.to_string(), "----0");

        let repr9 = Ternary::parse("+00");
        let repr4 = Ternary::parse("++");
        let repr13 = &repr9 + &repr4;
        let repr17 = &repr13 + &repr4;
        let repr34 = &repr17 + &repr17;//&a + &b

        assert_eq!(repr13.to_string(), "+++");
        assert_eq!(repr17.to_string(), "+-0-");
        assert_eq!(repr34.to_string(), "++-+");


        let repr9 = Ternary::parse("+--");
        let repr4 = Ternary::parse("+0+");//a + b
        let result=repr9 + repr4;
        assert_eq!(result.to_dec(), 15);

        let repr9 = Ternary::parse("+--");
        let repr4 = Digit::P;//&a + b
        let result=&repr9 + repr4;
        assert_eq!(result.to_dec(), 6);


        let repr34 = Ternary::parse("+00");
        let repr4 = Ternary::parse("++");
        let repr30 = &repr34 - &repr4;//&a - &b
        assert_eq!(repr30.to_dec(), 5);
        assert_eq!(repr30.to_string(), "+--");

        let repr30 = repr34 - repr4;//a - b
        assert_eq!(repr30.to_string(), "+--");

        let repr9 = Ternary::parse("+++");
        let repr4 = Digit::P;//&a - b
        let result=&repr9 - repr4;
        assert_eq!(result.to_dec(), 12);


        let repr34 = Ternary::parse("+00");
        let repr4 = Ternary::parse("++");
        let repr120 = &repr34 * &repr4;//&a * &b
        assert_eq!(repr120.to_dec(), 36);
        assert_eq!(repr120.to_string(), "++00");


        let repr34 = Ternary::parse("++++");
        let repr4 = Ternary::parse("++");
        let repr120 = &repr34 / &repr4;//&a / &b
        assert_eq!(repr120.quotient.to_string(), "+0+");
        println!("商和余数:{:?}{:?}",repr120.quotient,repr120.remainder);
    }

    #[test]
    fn test_ternary_choose() {
        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0000");
        assert_eq!(bitwise.to_string(), "0000");

        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0+00");
        assert_eq!(bitwise.to_string(), "0+00");

        let bitwise = &Ternary::parse("+000") | &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "+000");

        let bitwise = &Ternary::parse("+000") & &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "000-");

        let bitwise = &Ternary::parse("+000") ^ &Ternary::parse("000+");
        assert_eq!(bitwise.to_string(), "0000");


        let t1 =Ternary::parse("+++");
        let t2 =Ternary::parse("++0");

        if t1 > t2 {
            println!("t1 is greater than t2");
        } else if t1 < t2 {        // 还支持>=、<= 操作
            println!("t1 is less than t2");
        } else {
            println!("t1 and t2 are equal");
        }

        if t1!=t2{
            println!("t1 is not equl t2");
        }else if t1 == t2 {
            println!("t1 is  equl t2");
        }



    //     let b = !a;    // 自动走 Not 所有权版本
    //     println!("{:?}",b);
    //     let c = !&b;   // 借用版本，避免拷贝
    //     println!("{:?}",c);
    //     let d = -b;    // 使用 Neg 所有权版本
    //     let e = -&d;   // 使用 Neg 借用版本




    // let t3=!&(t1.tor(&t2));
    // let t5=t1.tnor(&t2);
    
    // //对输入取反
    // let t3=t1.tnand(&t2);
    // let (a,b)=(!&t1,!&t2);
    // let t5=a.tor(&b);

    //对输入取反
    //let t3=t1.tnand(&t2);
    //let t5 = (-&t1).tor(&(-&t2));
    //let t5 = (-&t1).tor(&- &t2);


   // let t5=(-t1).tor(&(-t2));

    // //对输入取反
    // let t3=t1.tnand(&t2);
    // let t5 = (!t1).tor(&!t2);

    }


}







