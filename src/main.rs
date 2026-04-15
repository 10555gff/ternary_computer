use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {
    // let a = Trit32::from_dec(92261865195136);
    // let b = Trit32::from_dec(2924);
    // // let a = Trit32::from_dec(5);
    // // let b = Trit32::from_dec(2);

    // let r =a/b;
    
    // println!("{},{}",a,b);


    // println!("{},{}",a.to_dec(),b.to_dec());

    // println!("{}",r);
    // println!("{},{}",r.quotient.to_dec(),r.remainder.to_dec());



    let a = trits!("0000_01TT");
    let b = trits!("0000_001T");
    println!("Div:{}",(a / b));
   
}