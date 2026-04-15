use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {
    let a = Trit8::from_dec(5);
    let b = Trit8::from_dec(3);
    println!("Div:{}",(a / b));
    println!("-----------------------");

    let a = Trit8::from_dec(5);
    let b = Trit8::from_dec(2);
    println!("Div:{}",(a / b));
    println!("-----------------------");

    let a = Trit8::from_dec(-5);
    let b = Trit8::from_dec(-2);
    println!("Div:{}",(a / b));
    println!("-----------------------");

    let a = Trit8::from_dec(5);
    let b = Trit8::from_dec(-2);
    println!("Div:{}",(a / b));
    println!("-----------------------");

    let a = Trit8::from_dec(-5);
    let b = Trit8::from_dec(2);
    println!("Div:{}",(a / b));
    println!("-----------------------");


    let a = Trit32::from_dec(92261865195136);
    let b = Trit32::from_dec(2924);
    let r =a/b;
    println!("{},{}",r.quotient.to_dec(),r.remainder.to_dec());
    println!("-----------------------");

    let a = trits!("00+0_++0+");
    let b = trits!("0000_0-0+");
    println!("Div:{}",(a / b));


    let a = trits!("0+0+");
    let b = trits!("00+-");
    println!("Div:{}",(a / b));

    let a = trits!("00++");
    let b = trits!("00+-");
    println!("Div:{}",(a / b));

    let a = trits!("0+0+");
    let b = trits!("0+--");
    println!("Div:{}",(a / b));

    let a = trits!("0+00");
    let b = trits!("00+-");
    println!("Div:{}",(a / b));

    let a = trits!("++++");
    let b = trits!("0+0-");
    println!("Div:{}",(a / b));

    let a = Trit4::from_dec(40);
    let b = Trit4::from_dec(5);
    let r =a/b;
    println!("{},{},{}",a,b,r);
    println!("{},{}",r.quotient.to_dec(),r.remainder.to_dec());
    println!("-----------------------");

    let a = Trit32::from_dec(58192762);
    let b = Trit32::from_dec(8);
    let r =a/b;
    println!("{},{},{}",a,b,r);
    println!("{},{}",r.quotient.to_dec(),r.remainder.to_dec());
    println!("-----------------------");
}