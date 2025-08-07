use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::Digit;
use ternary_arithmetic::ternary_utils::string_calculate;

// fn main() {
//     
//     let b = Ternary::parse("+-+-+");
//     let result=&a + &b;
//     result.digits_print();
// }



fn main() {
    // let s1 = "1234891588";
    // let s2 = "000";

    // let sub = string_calculate::decimal_subtractor(s1, s2);
    // println!("{s1}-{s2}={sub}");

    // let add = string_calculate::decimal_adder(s1, s2);
    // println!("{s1}+{s2}={add}");

    // let mul = string_calculate::decimal_multiply(s1, s2);
    // println!("{s1}*{s2}={mul}");

    let a = Ternary::parse("+-+-+-+-+++-++--++--0+++-++-+-++-++0+-0");
    println!(" a:{:?}",a.to_dec());
    println!("b:{:?}",a.to_dec_str());


}


