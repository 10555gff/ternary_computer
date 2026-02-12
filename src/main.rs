use ternary_arithmetic::ternary_asm::asm_utils;

fn main() -> std::io::Result<()> {
    asm_utils::write_tasm()?;  
    asm_utils::write_tbin()?;     
    asm_utils::read_tbin()?;     
    Ok(())
}
