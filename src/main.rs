use ternary_arithmetic::ternary_asm::asm_utils;

fn main() {
    let b=asm_utils::write_tasm();
    asm_utils::assemble();
    asm_utils::read();
}
