use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {
    // let a = trits!("T010_T010_T010_T010_T010_T010_T010_T010");
    // let b = trits!("---0_---0_---0_---0_---0_---0_---0_---0");
    // let c = trits!("0000_0000_0000_0000_0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0_+++0_+++0_+++0_+++0");

    // let a = trits!("T010_T010_T010_T010");
    // let b = trits!("---0_---0_---0_---0");
    // let c = trits!("0000_0000_0000_0000");
    // let d = trits!("+++0_+++0_+++0_+++0");

    // let a = trits!("T010_T010");
    // let b = trits!("---0_---0");
    // let c = trits!("0000_0000");
    // let d = trits!("+++0_+++0");

    let a = trits!("1101");
    let b = trits!("1011");

    let result = a.mul(b);
    println!("fff:{}",result);

}