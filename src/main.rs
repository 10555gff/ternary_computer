use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::{Digit,DibitLogic};

fn main() {

    let a:u8 = 0b00_10_10_10;
    let b:u8 = 0b00_01_00_00;
    
    //println!("d = {:08b}", d);
    let r=a.dibit_gate_full(&b,Digit::N);
    // let r =a.dibit_tand(d);
    // let r1 =b.dibit_tand(d);
    // let r2 =c.dibit_tand(d);

   // println!("{:?}",r);
    r.0.digits_print_t();
    r.1.digits_print_t();
     //r.digits_print_t();
    // r1.digits_print_t();
    // r2.digits_print_t();

    // let a = Ternary::parse("++");
    // let b = Ternary::parse("++");
    // let result=&a + &b;
    // result.digits_print();

    // // let result=&a - &b;
    // // result.digits_print_t();

    // // let result=&a * &b;
    // // result.digits_print();

    // let a = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-++-++++++++++++++-++++-+++--++++--0+++-++-+-++-++0+-0");
    // let b = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-+0+-0");
    // let c=&a / &b;
    //  println!("{}/{}={}",a.to_dec(),b.to_dec(),c.quotient.to_dec());
}
