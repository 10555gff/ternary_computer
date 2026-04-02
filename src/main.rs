use trit_macro::trits;
use ternary_arithmetic::trit::{Trit4,Trit8,Trit16,Trit32};
use ternary_arithmetic::ternary_cpu::asm_utils;


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

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM0)?;
    asm_utils::write_tbin()?;
    asm_utils::run_from_tbin()?;
    Ok(())
}
