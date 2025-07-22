mod ternary_utils;
use ternary_utils::logical_calculate::Digit;
use ternary_utils::ternary_io::Ternary;


fn main() {
     let aa=Digit::P;
//     let b=Digit::P;
//     let c=Digit::P;
//     let d=a.full_adder(b, c);

//    println!("result1 :{:?}",d);

//     let digits = vec![Digit::N, Digit::Z, Digit::P];
//     println!("result :{:?}",a.tor(b));



    // let t1 = Ternary::parse("00+++");

    //let c=Ternary::from_dec(-5);
    // let t2 = Ternary::new(vec![1, 0, 2]); // + 0 -
    // println!("{:?}{:?}",t1,t2);

    //println!("{}",t1.to_dec());
    //println!("{:?}",c.to_string());
    //c.digits_print();

    let repr9 = Ternary::parse("+00");
    let repr4 = Ternary::parse("++");
    let repr13 = &repr9 + &repr4;
    let repr17 = &repr13 + &repr4;
    let repr34 = &repr17 + &repr17;

    repr13.digits_print();
    repr17.digits_print();
    repr34.digits_print();

    
}
