use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::Digit;
use ternary_arithmetic::dibit_logic::{self, DibitLogic};


// fn get_2bit<T>(x: T, i: usize) -> u8
// where
//     T: Into<u64> + Copy,
// {
//     let v = x.into();
//     ((v >> (i * 2)) & 0b11) as u8
// }



// fn main() {
//     let a: u8 = 0b10_01_11_00;
//     let b: u16 = 0b10_01_11_00_01_11_00;
//     let c: u32 = 0b10_01_11_00_01_11_00_01_11_00;

//     println!("{:?}", (0..4).map(|i| get_2bit(a, i)).collect::<Vec<_>>());
//     println!("{:?}", (0..8).map(|i| get_2bit(b, i)).collect::<Vec<_>>());
//     println!("{:?}", (0..16).map(|i| get_2bit(c, i)).collect::<Vec<_>>());
// }



fn main() {
    let a: u16 = 0b0101_0000_0101_1010;
    a.digits_print_t();
//     //println!("d = {:08b}", d);
//     // let r=a.dibit_adder_u32(b,Digit::N);
//     // let r =a.dibit_tand(d);
//     // let r1 =b.dibit_tand(d);
//     // let r2 =c.dibit_tand(d);

//    // println!("{:?}",r);
//     // r.0.digits_print_t();
//     // r.1.digits_print_t();
//      //r.digits_print_t();
//     // r1.digits_print_t();


//     // let a = Ternary::parse("++");
//     // let b = Ternary::parse("++");
//     // let result=&a + &b;
//     // result.digits_print();

//     // // let result=&a - &b;
//     // // result.digits_print_t();

//     // // let result=&a * &b;
//     // // result.digits_print();

//     // let a = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-++-++++++++++++++-++++-+++--++++--0+++-++-+-++-++0+-0");
//     // let b = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-+0+-0");
//     // let c=&a / &b;
//     //  println!("{}/{}={}",a.to_dec(),b.to_dec(),c.quotient.to_dec());
}
