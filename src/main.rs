use ternary_arithmetic::ternary_cpu::asm_utils;

//REGS: REG0[TT]、REG1[T0]、REG2[T1]、REG3[0T]、REG4[00]、REG5[01]、REG6[1T]、REG7[10]、REG8[11]

pub static PROGRAM0: &[&str] = &[
    "0000_00TT_TTT0",// LOAD REG0
    "0100_00TT_00T0",// COPY REG0,REG1

    "0000_00TT_0000",// LOAD REG0
    "0100_00TT_00T1",// COPY REG0,REG2

    "0000_00TT_1110",// LOAD REG0
    "0100_00TT_000T",// COPY REG0,REG3

    "0000_00TT_T010",// LOAD REG0

    // "1T00_00T0_00TT",// CALC REG1,calctype
    // "1T00_00T1_00TT",// CALC REG2,calctype
    // "1T00_000T_00TT",// CALC REG3,calctype

    "1T00_00T0_00T0",// CALC REG1,calctype
    "1T00_00T1_00T0",// CALC REG2,calctype
    "1T00_000T_00T0",// CALC REG3,calctype
];

pub static PROGRAM1: &[&str] = &[
    "0000_000T_0010",// LOAD REG3

    //cout-1,until cout=0.
    "0000_00TT_000T",// LOAD REG0
    "1T00_000T_0011",// CALC REG3,SUB

    "1000_0001_00T0", //COND REG3>0
];

pub static PROGRAM2: &[&str] = &[
    //cout=3
    "0000_000T_0010",// LOAD REG3

    //cout-1,until cout=0.
    "0000_00TT_000T",// LOAD REG0
    "1T00_000T_0011",// CALC REG3,SUB

    //5+5+5=15
    "0000_00TT_01TT",// LOAD REG0
    "1T00_0011_0011",// CALC REG8,ADD

    "1000_0001_0T11", //COND REG3>0,0T11
];


fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM2)?;
    asm_utils::write_tbin()?;    
    asm_utils::read_tbin()?;     
    asm_utils::run_from_tbin()?;
    Ok(())
}

