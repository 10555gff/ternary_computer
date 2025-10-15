use ternary_arithmetic::logical_calculate::{Digit,DibitLogic};

fn main() {
    let a:u8 = 0b10_00_01_00;

    let b = 0b10_10_10_00;
    let c = 0b00_00_00_00;
    let d = 0b01_01_01_00;

    let r =a.dibit_tand(b);

    println!("a = {:08b}", a);
    println!("b = {:08b}", b);
    // println!("r = {:08b}", r);
    r.digits_print_t();

}
