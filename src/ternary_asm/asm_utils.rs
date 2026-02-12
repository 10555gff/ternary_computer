use std::fs::File;
use std::io::{Write, Read, BufWriter, BufRead, BufReader};

pub static LINES: &[&str] = &[
    "0001T1100T01",
    "000000000001",
    "0000001T1111",
    "0111TTT1111T",
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









pub fn read_tbin() -> std::io::Result<()> {
    let mut buf = std::fs::read("prog.tbin")?;

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


// pub fn read_tbin() -> std::io::Result<()> {
//     let file = File::open("prog.tbin")?;
//     let mut reader = BufReader::new(file);

//     let mut buf = Vec::new();
//     reader.read_to_end(&mut buf)?;// 读取整个文件到内存

//     println!("Read {} bytes", buf.len());

//     // 每 3 bytes = 1 instruction
//     for (i, inst) in buf.chunks_exact(3).enumerate() {
//         println!(
//             "INST {}: {:08b} {:08b} {:08b}",
//             i,
//             inst[0], inst[1], inst[2]
//         );
//     }

//     Ok(())
// }


// while !halted {
//     let inst = &mem[pc..pc+3];
//     pc += 3;

//     decode_execute(inst);
// }
