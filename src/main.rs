use ternary_arithmetic::ternary_asm::asm_utils;
/*  
    [1 bytes, 2 bytes, 3 bytes]
    Opcode: Imm[00]、Copy[01]、Calc[1T]、Condition[10]
    Imm Load dest=2bytes and val= 3bytes value.
    Copy Src= 2bytes value and dest=3bytes value.
    Calc Src= 2bytes value and calcType=3bytes value, REGS[src]=REG0 ctype REGS[src].
    Condition jump_type = 2bytes value and offset= 3bytes value,REG3 value and jump_type Decide to jump,then new_pc=pc + offset.

    REGS: REG0[TT]、REG1[T0]、REG2[T1]、REG3[0T]、REG4[00]、REG5[01]、REG6[1T]、REG7[10]、REG8[11]
    CalType: tor[TT]、tand[T0]、tnor[T1]、tnand[0T]、txor[00]、tnxor[01]、add[1T]、tany[10]、tnany[11]
*/

pub static PROGRAM0: &[&str] = &[
    "0000_00T0_TTT0",// LOAD REG1
    "0000_00T1_0000",// LOAD REG2
    "0000_000T_1110",// LOAD REG3
    "0000_00TT_T010",// LOAD REG0

    // "1T00_00T0_00TT",// CALC REG1,tor
    // "1T00_00T1_00TT",// CALC REG2,tor
    // "1T00_000T_00TT",// CALC REG3,tor

    "1T00_00T0_00T0",// CALC REG1,tand
    "1T00_00T1_00T0",// CALC REG2,tand
    "1T00_000T_00T0",// CALC REG3,tand
];

pub static PROGRAM1: &[&str] = &[
    //cout=3
    "0000_000T_0010",// LOAD REG3

    //cout-1,until cout=0.
    "0000_00TT_000T",// LOAD REG0
    "1T00_000T_001T",// CALC REG3,ADD

    //5+5+5=15
    "0000_00TT_01T0",// LOAD REG0
    "1T00_0011_001T",// CALC REG8,ADD

    "1000_0001_0T11", //COND REG3>0,0T11
];

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm(PROGRAM1)?;
    asm_utils::write_tbin()?;    
    asm_utils::read_tbin()?;     
    asm_utils::run_from_tbin()?;
    Ok(())
}

