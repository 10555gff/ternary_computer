use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {
    // let a = trits!("T010_T010_T010_T010_T010_T010_T010_T010");
    // let b = trits!("---0_---0_---0_---0_---0_---0_---0_---0");
    // let c = trits!("0000_0000_0000_0000_0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0_+++0_+++0_+++0_+++0");

    // let a = trits!("0000_0000_0000_1101");
    // let b = trits!("0000_0000_0000_1011");
    // let c = trits!("0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0");


    // let a = trits!("00+0_++0+");
    // let b = trits!("0000_0+0-");
    // let re = a.div(b);//280/8
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0000_++++");
    // let b = trits!("0000_0+0-");
    // let re = a.div(b);//40/8
    // println!("quotient:{},remainder:{}",re.0,re.1);



    // let a = trits!("0000_0-0-");
    // let b = trits!("0000_00-+");
    // let re = a.div(b);//-10/-2
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0000_0+0+");
    // let b = trits!("0000_00-+");
    // let re = a.div(b);//10/-2
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0000_0-0-");
    // let b = trits!("0000_00+-");
    // let re = a.div(b);//-10/2
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0000_0+0+");
    // let b = trits!("0000_00+0");
    // let re = a.div(b);//10/3
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("000+_-++-");
    // let b = trits!("0000_0+--");
    // let re = a.div(b);//65/5
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0000_+-++");
    // let b = trits!("0000_0-++");
    // let re = a.div(b);//22/5
    // println!("quotient:{},remainder:{}",re.0,re.1);

    // let a = trits!("0+0+");
    // let b = trits!("00-+");
    // let re = a / b;//10/2
    //println!("{}",re);

    // let a = trits!("00+0_++0+");
    // let b = trits!("0000_0+0-");
    // let re = a /b;//280/8
    // println!("{}",re);

    // let a = trits!("0000_0000_00+0_++0+");
    // let b = trits!("0000_0000_0000_0+0-");
    // let re = a /b;//280/8
    // println!("{}",re);

    // let a = trits!("0000_0000_0000_0000_0000_0000_00+0_++0+");
    // let b = trits!("0000_0000_0000_0000_0000_0000_0000_0+0-");
    // let re =a/b;//280/8
    // println!("{}",re);


    let res =Trit32::from_dec(926510094425919);
    println!("{}",res);


}