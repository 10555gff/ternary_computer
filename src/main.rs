use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {

    let mut t = trits!("T010_TTTT");
    //let b = trits!("T010_TTTT_1111_TT0T_T010_1001_0011_TTTT");

    // t.set(2, 1);
    t.set(1, 3);

    // println!("{}", t.get(0));
    // println!("{}", t.get(1));
    // println!("{}", t.get(2));
    // println!("{}", t.get(3));
    // println!("{}", t.get(4));
    // println!("{}", t.get(5));
    // println!("{}", t.get(6));
    // println!("{}", t.get(7));

    
    println!("{}", t);
    println!("{}", Trit8(t.tneg(7)));


    // println!("{}", b);
    // let my_word: u8 = 0b11011000; // 二进制表示：11(3), 01(1), 10(2), 00(0)
    
    // // 调用函数
    // let result = bit_utils::fmt(my_word);
    // println!("输入字节: {:08b}", my_word);
    // println!("解码结果: {}", result);
}




// fn main() {
//     let a = trits!("T010");
//     let b = trits!("---0_0000");


//     // let mut t = Trit4::new(0);



//     // let result = a.tor(b);
//     // println!("{}", result);


//     // let a = trits!("T010");
//     // let b = trits!("---0");
//     // let c = trits!("0000");
//     // let d = trits!("+++0");
//     // //CTYPE: [0]tor [1]tand [2]tnor [3]tnand [4]txor [5]tnxor [6]add [7]tcons [8]tany [9]tnany
//     // let ctype=0;
    
//     // let result1 =a.gate_core(b,ctype);
//     // let result2 =a.gate_core(c,ctype);
//     // let result3 =a.gate_core(d,ctype);


//     // println!("{}",result1);
//     // println!("{}",result2);
//     // println!("{}",result3);
// }