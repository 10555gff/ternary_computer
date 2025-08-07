use ternary_arithmetic::ternary_io::Ternary;
// use ternary_arithmetic::logical_calculate::Digit;
use ternary_arithmetic::ternary_utils::string_calculate;


fn main() {
    let s1 = "369245";
    let s2 = "155155";

    let sub = string_calculate::decimal_subtractor(s1, s2);
    println!("{s1}-{s2}={sub}");

    let add = string_calculate::decimal_adder(s1, s2);
    println!("{s1}+{s2}={add}");

    let mul = string_calculate::decimal_multiply(s1, s2);
    println!("{s1}*{s2}={mul}");

    let a = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-++-++++++++++++++-++++-+++--++++--0+++-++-+-++-++0+-0");
    let b = Ternary::parse("+-+-+-+-+++-+++-0++-000++--+-++-+0+-0");
    let c=&a / &b;
    println!("{}/{}={}",a.to_dec(),b.to_dec(),c.quotient.to_dec());

}


