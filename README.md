# Ternary Arithmetic

**高性能平衡三进制（Balanced Ternary）算术库 + 三进制 CPU 模拟器**，纯 Rust 实现。

每个 **trit**（三进制位）用 **2 bit** 存储在 `u8` 中：

```
T (-1) → 10
0      → 00
1 (+1) → 01
```

`Trit4` 把 **4 个 trit** 打包进 **1 个 u8**，实现硬件级 SIMD 风格运算，无堆分配，极致性能。

## ✨ 核心特性

### 🧠 平衡三进制逻辑门（并行位运算）
- `tor` / `tand` / `tnor` / `tnand`
- `txor` / `tnxor`
- `tany` / `tnany`
- `tcons` / `tncons`
- `tsum`（三值求和门）
- `add` / `sub`（带进位全加器，使用预计算查找表）

全部通过 `gate_core(code)` 统一调用（`code` 0~9）。

### ➕ 算术与工具函数
- 完整加法器 + 减法（`+` `-` 运算符重载）
- 取反（`!` / `-`）
- 十进制转换 `to_dec()`
- 符号提取 `to_sign()`
- 比较运算（`Ord` 实现，支持 `> < >= <= == !=`）
- 移位运算（`<<` `>>`）
- 位级操作：`get()`、`set()`、`clear()`、`toggle()`、`get_all()`

### 🖥 Ternary CPU 模拟器（T80）
- **9 个寄存器**（R0 为累加器，R3 为条件码寄存器）
- **固定 3 字节指令**（24 bit）
- 支持：立即数加载、寄存器复制、Calc 计算、条件跳转、Halt
- 完整汇编工具链（`.tasm` → `.tbin` 编译 + 执行）

## 📦 安装

```bash
cargo add ternary_arithmetic trit_macro
```

## 🚀 使用示例

### 1. 逻辑门与 gate_core（推荐写法）

```rust
use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

fn main() {
    let a = trits!("T010");
    let b = trits!("---0");   // 等价于 TTT0

    println!("a | b = {}", a.tor(b));
    println!("a & b = {}", a.tand(b));

    // 使用统一入口（最灵活）
    let result = a.gate_core(b, 0); // 0=tor
    println!("gate_core(0) = {}", result);
}
```

### 2. 算术运算与运算符重载

```rust
use ternary_arithmetic::ternary_cpu::logical_alu::Trit4;

let a = Trit4(0b00000001);  // 1
let b = Trit4(0b00000010);  // -1（T）

println!("{} + {} = {}", a, b, (a + b).sum);
println!("{} - {} = {}", a, b, (a - b).sum);
println!("a.to_dec() = {}", a.to_dec());
```

### 3. Ternary CPU 汇编运行

```rust
use ternary_arithmetic::ternary_asm::asm_utils;

static PROGRAM: &[&str] = &[
    "0000_0000_0001",   // Imm: R0 ← 1
    "0100_0000_0001",   // Copy: R1 ← R0
    "0000_0000_001T",   // Imm: R0 ← -1
    "1T00_0001_001T",   // Calc: R1 ← R1 + R0（加法）
    "1000_0001_0001",   // Condition: 如果 R3 > 0 则跳转
    "1111_1111_1111",   // Halt
];

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM)?;
    asm_utils::write_tbin()?;
    asm_utils::run_from_tbin()?;
    Ok(())
}
```

## 📋 T80 CPU 指令集参考表（最重要）

| Opcode | 指令类型     | Byte2          | Byte3              | 功能说明 |
|--------|--------------|----------------|--------------------|----------|
| `0x00` | **Imm**      | -              | 立即数 (u8)       | R0 ← 立即数 |
| `0x10` | **Copy**     | src 索引       | dst 索引           | R[dst] ← R[src] |
| `0x60` | **Calc**     | src 索引       | ctype (0~9)        | R[src] ← R0 `gate_core` R[src] |
| `0x40` | **Condition**| jump_type (0~7)| offset (i8)        | 根据 R3 判断跳转 |
| `0xFF` | **Halt**     | -              | -                  | 程序停止 |

**Calc ctype 对应表**（0~9）：
0=tor、1=tand、2=tnor、3=tnand、4=txor、5=tnxor、**6=加法**、7=tncons、8=tany、9=tnany

## 🧩 设计哲学

- 完全模拟**硬件级三值逻辑门**与全加器
- 全部使用**查找表 + 位掩码 + 无分支**运算
- 为教学、实验、探索“非二进制计算机”而生
- 零依赖、高性能、无 unsafe

## 📜 License

MIT License
