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
use ternary_arithmetic::logical_calculate::Digit;

fn main() {
    let a = Ternary::parse("+0-+-+");
    let b = Ternary::parse("+-+-+");
    let result=&a + &b;
    result.digits_print();
}

```

## License

This project is licensed under the MIT License.
