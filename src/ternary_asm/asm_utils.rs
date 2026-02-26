use std::fs::File;
use std::io::{Write, BufWriter, BufRead, BufReader};
use crate::ternary_cpu::logical_cpu::T80CPU;

pub static LINES: &[&str] = &[
    "0000T1100T01",
    "0100T1100T01",
    "1T00T1100T01",
    "1100T1100T01"
];

pub fn write_tasm() -> std::io::Result<()> {
    let file=File::create("prog.tasm")?;
    let mut buf = BufWriter::new(file);

    for l in LINES {
        writeln!(buf, "{l}")?;
    }
    Ok(())
}

/// 把单个 trit 转 2-bit
fn trit_index(c: u8) -> u8 {
    match c {
        b'T' => 0b10, // -1
        b'0' => 0b00, // 0
        b'1' => 0b01, // +1
        _ => unreachable!(),
    }
}

/// 把一条 trit 指令转成字节流
fn pack_trits(trits: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut cur_byte = 0u8;
    let mut count = 0;
    const SHIFT: [u8; 4] = [6, 4, 2, 0];

    for &c in trits.as_bytes() {
        let bits = trit_index(c);
        cur_byte |= bits << SHIFT[count];// 每 byte 放 4 trits
        count += 1;

        if count == 4 {
            bytes.push(cur_byte);
            cur_byte = 0;
            count = 0;
        }
    }

    if count != 0 {
        bytes.push(cur_byte);
    }

    bytes
}


pub fn write_tbin() -> std::io::Result<()> {
    // 打开 .tasm 文件
    let reader = BufReader::new(File::open("prog.tasm")?);

    // 输出 .tbin 文件
    let out_file = File::create("prog.tbin")?;
    let mut buf = BufWriter::new(out_file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // 忽略空行和注释
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let packed = pack_trits(line);
        buf.write_all(&packed)?;
    }

    println!("Assemble done → prog.tbin");
    Ok(())
}


pub fn run_from_tbin() ->std::io::Result<()> {
    // 读取二进制程序
    //把文件 prog.tbin全部内容一次性读入内存，得到一个 Vec<u8>
    let mem = std::fs::read("prog.tbin")?;

    // 创建 CPU，内存就是刚刚读到的字节
    let mut cpu = T80CPU::new(mem);

    // // 可选：初始化一些寄存器值（模拟 INPUT 或初始数据）
    // cpu.regs.write(0, 0b01_00_00_00);
    // cpu.regs.write(1, 0b10_01_00_00);

    // let r1 = cpu.regs.read(1);
    // println!("{}",r1);

    // println!("开始执行程序... PC 从 0 开始");
    // let mut a=cpu.fetch();
    // let mut a=cpu.fetch();
    // let mut a=cpu.fetch();
    // let mut a=cpu.fetch();
    // 
    // 运行直到结束（或你自己加 halted 标志）
    cpu.run();

    println!("执行完成");

    // // 可选：打印最终寄存器状态
    // for i in 0..6 {
    //     println!("REG{} = {:?}", i, cpu.regs.read(i));
    // }

    Ok(())
}





pub fn read_tbin() -> std::io::Result<()> {
    let buf = std::fs::read("prog.tbin")?;

    let mut pc = 0;
    let inst_size = 3; // 12 trits = 3 bytes

    while pc + inst_size <= buf.len() {
        let inst = &buf[pc..pc+3];

        println!(
            "PC={} INST={:08b} {:08b} {:08b}",
            pc,
            inst[0], inst[1], inst[2]
        );

        pc += inst_size; // next instruction
    }

    Ok(())
}

// while !halted {
//     let inst = &mem[pc..pc+3];
//     pc += 3;

//     decode_execute(inst);
// }
