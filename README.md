[![Rust](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml/badge.svg)](https://github.com/10555gff/ternary_computer)

# Ternary Arithmetic

A Rust library use array logic gate to arithmetic.
a numeral system with digits `-1`, `0`, and `+1`.

## Installation

Add this to your `Cargo.toml`:

```rust
cargo add ternary_arithmetic
```

### Example

```rust
use ternary_arithmetic::ternary_io::Ternary;
use ternary_arithmetic::logical_calculate::{Digit,DibitLogic};

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

```

## License

This project is licensed under the MIT License.
