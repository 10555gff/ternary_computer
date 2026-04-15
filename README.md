# Ternary Arithmetic

**A Rust library use array logic gate to arithmetic. a numeral system with digits -1, 0, and +1.

Each trit is stored using 2 bits inside u8/u16/u32/u64:

```
T (-1) → 10
0      → 00
1 (+1) → 01
```

## ✨ Features

### 🧠 Balanced Ternary Logic Gates
- `tneg`
- `txor` / `tnxor`
- `tor` / `tnor` / `tand` / `tnand`

- `tany` / `tnany` / `tcons` / `tncons` / `tsum`

## 📦 Installation

```bash
cargo add ternary_arithmetic trit_macro
```

## 🚀 Example – Logic Gates

```rust
use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};

fn main() {
    let c = Trit32::ZERO;
    let a = trits!("T010_T010_T010_T010_T010_T010_T010_T010");
    let b = trits!("---0_---0_---0_---0_---0_---0_---0_---0");
    let d = trits!("+++0_+++0_+++0_+++0_+++0_+++0_+++0_+++0");
    println!("{}",a.tand(b));
    println!("{}",a.tand(c));
    println!("{}",a.tand(d));
    println!("-----------------------");

    let a = trits!("T010_T010_T010_T010");
    let b = trits!("---0_---0_---0_---0");
    println!("Add:{}",(a + b));
    println!("Sub:{}",(a - b));
    println!("Mul:{}",(a * b));
    println!("DiV:{}",(a / b));
    println!("-----------------------");

    let a = trits!("T010_T010");
    let b = trits!("+++0_---0");
    let c = trits!("1111_1111");
    let d = Trit8::from_dec(3280);
    println!("a:{}",(a >> 1));
    println!("b:{}",(a << 2));
    println!("a:{},b:{},c:{},d:{}",a,b,c.to_dec(),d);
    println!("-----------------------");

    let a = Trit32::from_dec(58192762);
    let b = Trit32::from_dec(8);
    let r =a/b;
    println!("{},{},{}",a,b,r);
    println!("{},{}",r.quotient.to_dec(),r.remainder.to_dec());
    println!("-----------------------");

    let a = trits!("T011");
    let b = trits!("1111");
    let c= Trit4(b.clear(2));
    println!("{},{}",-a,!b);
    println!("a:{},c:{}",a.get(3),c);
    println!("{},{},{}",(a | b),(a & b),(a ^ b));
    println!("-----------------------");

}

```


## 📜 License

MIT License
