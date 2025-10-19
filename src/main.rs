use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::Digit;
use ternary_arithmetic::dibit_logic::{self, DibitLogic};


fn main() {
    let a:u8 = 0b10_10_10_10;
    let b:u8 = 0b00_00_00_00;
    let c:u8 = 0b01_01_01_01;
    let d:u8 = 0b10_00_01_00;
    
    //println!("d = {:08b}", d);
    let r =a.dibit_tand(d);
    let r1 =b.dibit_tand(d);
    let r2 =c.dibit_tand(d);
    r.digits_print_t();
    r1.digits_print_t();
    r2.digits_print_t();

    let a = Ternary::parse("+0-+-+");
    let b = Ternary::parse("+-+-+");
    let result=&a + &b;
    result.digits_print();

    let result=&a - &b;
    result.digits_print_t();

    let result=&a * &b;
    result.digits_print();

    let a = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-++-++++++++++++++-++++-+++--++++--0+++-++-+-++-++0+-0");
    let b = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-+0+-0");
    let c=&a / &b;
    println!("{}/{}={}",a.to_dec(),b.to_dec(),c.quotient.to_dec());
}
