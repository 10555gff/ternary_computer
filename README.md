# Ternary Arithmetic

[![Rust](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml/badge.svg)](https://github.com/10555gff/ternary_computer)

A high-performance **Balanced Ternary Arithmetic Library** implemented in Rust.  


Each `trit` is stored using **2 bits inside a `u8`**:

```
T (-1) → 10
0      → 00
1 (+1) → 01
```

A `Trit4` packs **4 trits into one `u8`**:

```
[T3 T2 T1 T0]  →  8 bits total
```

This allows:

* Bitwise SIMD-style operations
* Fast logical gate evaluation
* Efficient arithmetic without heap allocation
* Hardware-style gate simulation

---

## ✨ Features

### 🧠 Balanced Ternary Logic Gates

Implemented using array-based lookup tables:

* `tor`
* `tand`
* `tnor`
* `tnand`
* `txor`
* `tnxor`
* `tany`
* `tnany`
* `tcons`
* `tsum`
* `add`

All gates operate in parallel on packed trits.

---

### ➕ Arithmetic Support

* Balanced ternary addition
* Subtraction
* Unary negation
* Carry logic via full-adder tables
* Decimal conversion (`to_dec`)
* Sign detection (`to_sign`)

Operator overloading supported:

```rust
+  -  &  |  ^  <<  >>  !  -
>  <  >= <= == !=
```

---

### 🔧 Bit-level Trit Operations

* `get(index)`
* `set(index, value)`
* `clear(index)`
* `toggle(index)`
* `get_all()`

Predefined constants:

```rust
Trit4::NEG
Trit4::ZERO
Trit4::POS
Trit4::ALL_NEG
Trit4::ALL_POS
```

---

### 🖥 Ternary CPU & Assembly Support

Includes a minimal ternary CPU simulator:

* Register file (REG0 ~ REG8)
* Instruction types:

  * `Imm`
  * `Copy`
  * `Calc`
  * `Condition`
* Ternary assembly compiler
* Binary generator
* Binary runner

Supports writing `.tasm` → compiling → executing in a simulated ternary machine.

---

## 📦 Installation

Add this to your project:

```bash
cargo add ternary_arithmetic trit_macro
```

---

## 🚀 Example – Logic Gates

```rust
use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {
    let a = trits!("T010");
    let b = trits!("---0");

    let result = a.tor(b);
    println!("{}", result);


    let a = trits!("T010");
    let b = trits!("---0");
    let c = trits!("0000");
    let d = trits!("+++0");
    //CTYPE: [0]tor [1]tand [2]tnor [3]tnand [4]txor [5]tnxor [6]add [7]tcons [8]tany [9]tnany
    let ctype=0;
    
    let result1 =a.gate_core(b,ctype);
    let result2 =a.gate_core(c,ctype);
    let result3 =a.gate_core(d,ctype);


    println!("{}",result1);
    println!("{}",result2);
    println!("{}",result3);
}
```

---

## 🚀 Example – Raw Bit Representation

```rust
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {
    //T-->10  0-->00  1-->01
    let a:Trit4 = Trit4(0b10000100);
    let b:Trit4 = Trit4(0b10000100);

    // a.tor(b) a.tand(b) a.tnor(b) a.tnand(b) a.xor(b) a.nxor(b) a.cons(b) a.ncons(b) a.tany(b) a.nany(b) a.tsum(b) a.add(b,0)
    let result1 = a.gate_core(b,0);
    let result2 = a.tor(b);
    let result3 = a.tneg();

    println!("{}",result1);
    println!("{}",result2);
    println!("{}",result3);
    //---------------------------------------------------------------------------//

    let x=a.get(3);//0~3
    println!("{:08b}",x);

    let x=a.get_all();
    println!("{:08b}",x[3]);

    let mut a:Trit4 = Trit4(0b10000100);
    a.set(0,0b01);//index[0~3],val
    println!("{:08b}",a.0);
    
    let a:Trit4 = Trit4(0b10000100);
    let x =a.clear(1);
    println!("{:08b}",x);

    let a:Trit4 = Trit4(0b10000100);
    let x =a.toggle(3);
    println!("{:08b}",x);

    println!("{}",Trit4::NEG);
    println!("{}",Trit4::ZERO);
    println!("{}",Trit4::POS);
    println!("{}",Trit4::ALL_POS);
    println!("{}",Trit4::ALL_NEG);

}
```

---

## 🚀 Example – Arithmetic

```rust
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {
    let a:Trit4 = Trit4(0b00000001);
    let b:Trit4 = Trit4(0b00000001);

    println!("a value:{}",a);
    println!("b value:{}",b);

    let c =a + b; println!("{}",c);
    let c =a - b; println!("{}",c);

    let c =a & b; println!("{}",c);
    let c =a | b; println!("{}",c);
    let c =a ^ b; println!("{}",c);

    let c =a > b; println!("{}",c);
    let c =a < b; println!("{}",c);
    let c =a == b; println!("{}",c);

    let c =a >= b; println!("{}",c);
    let c =a <= b; println!("{}",c);
    let c =a != b; println!("{}",c);

    let c =a<<1;  println!("{}",c);
    let c =b>>1;  println!("{}",c);

    let c =-b;  println!("{}",c);
    let c =!b;  println!("{}",c);
    //---------------------------------------------------------------------------//

    let a:Trit4 = Trit4(0b00100100);
    let b:Trit4 = Trit4(0b00001001);

    let c =a.to_dec();  println!("{}",c);
    let c =b.to_sign();  println!("{}",c);
}
```

---

## 🚀 Example – CPU Assembly

```rust
use ternary_arithmetic::ternary_asm::asm_utils;
/*  
    [1 bytes, 2 bytes, 3 bytes]
    Opcode: Imm[00]、Copy[01]、Calc[1T]、Condition[10]
    Imm Load REG0= 3bytes value.
    Copy Src= 2bytes value and dest=3bytes value.
    Calc Src= 2bytes value and calcType=3bytes value, REGS[src]=REG0 ctype REGS[src].
    Condition jump_type = 2bytes value and offset= 3bytes value,REG3 value and jump_type Decide to jump,then new_pc=pc + offset.

    REGS: REG0[TT]、REG1[T0]、REG2[T1]、REG3[0T]、REG4[00]、REG5[01]、REG6[1T]、REG7[10]、REG8[11]
    CalType: tor[TT]、tand[T0]、tnor[T1]、tnand[0T]、txor[00]、tnxor[01]、add[1T]、tany[10]、tnany[11]
*/

//TOR Calc
pub static PROGRAM0: &[&str] = &[
    "0000_0000_TTT0",// LOAD REG0
    "0100_00TT_00T0",// COPY REG0,REG1

    "0000_0000_0000",// LOAD REG0
    "0100_00TT_00T1",// COPY REG0,REG2

    "0000_0000_1110",// LOAD REG0
    "0100_00TT_000T",// COPY REG0,REG3

    "0000_0000_T010",// LOAD REG0

    "1T00_00T0_00TT",// CALC REG1,calctype
    "1T00_00T1_00TT",// CALC REG2,calctype
    "1T00_000T_00TT",// CALC REG3,calctype

    // "1T00_00T0_00T0",// CALC REG1,calctype
    // "1T00_00T1_00T0",// CALC REG2,calctype
    // "1T00_000T_00T0",// CALC REG3,calctype
];
pub static PROGRAM1: &[&str] = &[
    //cout=3
    "0000_0000_0010",// LOAD REG0
    "0100_00TT_000T",// COPY REG0,REG3

    //cout-1,until cout=0.
    "0000_0000_000T",// LOAD REG0
    "1T00_000T_001T",// CALC REG3,SUB

    "1000_0001_00T0", //COND REG3>0,0T11
];
//While Add Calc
pub static PROGRAM2: &[&str] = &[
    //cout=3
    "0000_0000_0010",// LOAD REG0
    "0100_00TT_000T",// COPY REG0,REG3

    //cout-1,until cout=0.
    "0000_0000_000T",// LOAD REG0
    "1T00_000T_001T",// CALC REG3,SUB

    //5+5+5=15
    "0000_0000_01TT",// LOAD REG0
    "1T00_0011_001T",// CALC REG8,ADD

    "1000_0001_0T11", //COND REG3>0,0T11
];

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM2)?;
    asm_utils::write_tbin()?;    
    asm_utils::read_tbin()?;     
    asm_utils::run_from_tbin()?;
    Ok(())
}


```

---

## 🧩 Design Philosophy

This project aims to:

* Simulate hardware-level ternary logic
* Provide a clean arithmetic abstraction
* Enable ternary computer experimentation
* Explore non-binary computing models

All core logic is implemented using:

* Lookup tables
* Bit masks
* Branchless operations
* Pure `u8` arithmetic

---

## 📜 License

This project is licensed under the MIT License.
