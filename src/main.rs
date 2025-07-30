mod ternary_utils;
use ternary_utils::ternary_io::Ternary;
use ternary_utils::logical_calculate::Digit;


fn main() {
    let t1 =Ternary::parse("++00+++++++++0+++");
    let t2 =Ternary::parse("+0-");
    // let t5 =Ternary::parse("++++-0");
    // let t3 =t1==t2;
    // let t4=t1.trim_leading_zeros();
    // println!("{:?} ",t4);

//     if t1 >= t2 {
//     println!("t1 is greater than t2");
// } else if t1 <= t2 {
//     println!("t1 is less than t2");
// } else {
//     println!("t1 and t2 are equal");
// }

//     let t1 = Ternary::new(vec![Digit::P, Digit::N]);
// let t2 = Ternary::new(vec![Digit::P, Digit::N]);
// let t3 = Ternary::new(vec![Digit::N, Digit::Z]);


    // 
    let t3 =&t1 / &t2;
    println!("商和余数:{:?}",t3);


    let (a,b,c)=(t1.to_dec(),t2.to_dec(),t3.quotient.to_dec());
    println!("{}/{}={}",a,b,c);
//  //   let t3=t1.mul_base(&t2);
//     let t4=&t1 * &t2;
//     // t3.digits_print();
//     t4.digits_print();
//     println!("{}",t4.to_dec());


    
    // //对输出取反
    // let t3=!&(t1.tor(&t2));
    // let t5=t1.tnor(&t2);
    
    // //对输入取反
    // let t3=t1.tnand(&t2);
    // let (a,b)=(!&t1,!&t2);
    // let t5=a.tor(&b);

    //对输入取反
//     let t3=t1.tnand(&t2);
//     //let t5 = (-&t1).tor(&(-&t2));
//     let t5 = (-&t1).tor(&- &t2);


//    // let t5=(-t1).tor(&(-t2));

//     // //对输入取反
//     // let t3=t1.tnand(&t2);
//     // let t5 = (!t1).tor(&!t2);


//     t3.digits_print();
//     t5.digits_print();


//     let a = Ternary::parse_t("1T0");

//     let b = !a;    // 自动走 Not 所有权版本
//     println!("{:?}",b);
//     let c = !&b;   // 借用版本，避免拷贝
//     println!("{:?}",c);
//     let d = -b;    // 使用 Neg 所有权版本
//     let e = -&d;   // 使用 Neg 借用版本

   


//t1.digits_print();

    // let t3=&t1+t2;

    // t1.digits_print();
    // // t2.digits_print();

    // t3.digits_print();

    //let t2=t1.to_neg();
    //let c=t1.sign();

    //println!("{:?}",c);
    // println!("{:?}",t2);
    

    //let c=Ternary::from_dec(-5);
    // let t2 = Ternary::new(vec![1, 0, 2]); // + 0 -
    // println!("{:?}{:?}",t1,t2);

    //println!("{}",t1.to_dec());
    //println!("{:?}",c.to_string());
    //c.digits_print();

    // let repr9 = Ternary::parse("+00");
    // let repr4 = Ternary::parse("++");
    // let repr13 = &repr9 + &repr4;
    // let repr17 = &repr13 + &repr4;
    // let repr34 = &repr17 + &repr17;

    // repr13.digits_print();
    // repr17.digits_print();
    // repr34.digits_print();

    // let a = Ternary::from_dec(5);
    // let d = Digit::N; // -1
    // let result = &a + d; // 等价于 Ternary::from_dec(4)

    // result.digits_print();



    // let a=Digit::P;
    // let b=Digit::P;
    // let c=Digit::P;


    // let d=a.full_adder(b, c);

    // println!("result1 :{:?}",d.sum);


    // println!("result :{:?}",a.half_adder(b));

    // let a = Digit::P;
    // let b = Digit::N;
    // let result = a - b;
    // // println!("sum = {:?}, carry = {:?}", result.sum, result.carry);
    // println!("{:?}",result);

    // let test = Ternary::from_dec(18887455);
    // test.digits_print_t();
    // assert_eq!(test.to_string(), "++00--0--+-0++0+");
    
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;//会消耗左值

    // println!("{}",s2);
    // println!("{}",s3);
    // //println!("{}",s1);
//     let c1 = &a + &b; // 不消耗 a 和 b
// let c2 = a + b;   // 消耗所有权
}
