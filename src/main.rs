// use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::Digit;
use ternary_arithmetic::dibit_logic::DibitLogic;


fn main() {
    let a:u16 = 0b0101_0101_0101_0101;
    let b:u16 = 0b0100_0000_0000_0000;
    let r=a.dibit_adder(b, Digit::Z);

    r.0.digits_print_t();
    r.1.digits_print_t();


    // let a = Ternary::parse("+0-+-+");
    // let b = Ternary::parse("+-+-+");
    // let result=&a + &b;
    // result.digits_print();

    // let result=&a - &b;
    // result.digits_print_t();

    // let result=&a * &b;
    // result.digits_print();

    // let a = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-++-++++++++++++++-++++-+++--++++--0+++-++-+-++-++0+-0");
    // let b = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-+0+-0");
    // let c=&a / &b;
    // println!("{}/{}={}",a.to_dec(),b.to_dec(),c.quotient.to_dec());
}
