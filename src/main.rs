use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

// const ORDER: [i8; 3] = [0, 1, -1];
// const ORDER2: [u8; 3] = [1, 2, 0];


fn main() {
    // let a = trits!("T010_T010_T010_T010_T010_T010_T010_T010");
    // let b = trits!("---0_---0_---0_---0_---0_---0_---0_---0");
    // let c = trits!("0000_0000_0000_0000_0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0_+++0_+++0_+++0_+++0");

    // let a = trits!("T010_T010_T010_T010");
    // let b = trits!("---0_---0_---0_---0");
    // let c = trits!("0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0");

    let a = trits!("T010_T010");
    let b = trits!("1010_T010");
    // let c = trits!("0000_0000");
    // let d = trits!("+++0_+++0");

    // let t1 = trits!("++-0");
    // let t2 = trits!("++--");
    let ord = a.cmp(&b);
    println!("{:?}",ord);
    // let c:u8 =2;

    // let result = a.adder(b,c);
    // println!("Result0:{}{}",result.1,result.0);
    

    // for i in (0..4).rev() {
    //     let a = t1.get(i);
    //     let b = t2.get(i);
    //                 let k=val(a);
    //         println!("Result:{}",k);
    //     if a != b {

    //         //return
    //     }
    //     println!("Result:{}{}",a,b);
    // }



}