use std::fs::File;
use std::io::{self, Write, Read, BufWriter, BufRead, BufReader};

pub static LINES: &[&str] = &[
    "0001T1100T01",
    "000000000001",
    "0000001T1111",
    "0111TTT1100T",
];


/// 把单个 trit 转 2-bit
fn trit_to_bits(t: char) -> u8 {
    match t {
        'T' => 0b10,
        '0' => 0b00,
        '1' => 0b01,
        _ => panic!("非法 trit: {}", t),
    }
}




/// 把一条 trit 指令转成字节流
fn pack_trits(trits: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut cur_byte = 0u8;
    let mut count = 0;

    for c in trits.chars() {
        let bits = trit_to_bits(c);
        cur_byte |= bits << (6 - count * 2); // 每 byte 放 4 trits
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


pub fn write_tasm() -> io::Result<()> {
    let mut w = BufWriter::new(File::create("prog.tasm")?);
    for l in LINES {
        writeln!(w, "{l}")?;
    }
    Ok(())
}


pub fn assemble() -> io::Result<()> {
    let r = BufReader::new(File::open("prog.tasm")?);
    let mut w = BufWriter::new(File::create("prog.tbin")?);

    for line in r.lines().filter_map(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        w.write_all(&pack_trits(line))?;
    }

    println!("Assemble done.");
    Ok(())
}








pub fn read() -> std::io::Result<()> {
    // 打开文件
    let file = File::open("prog.tbin")?;
    //let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?; // 读取整个文件到内存

    println!("Read {} bytes", buf.len());
    for (i, &b) in buf.iter().enumerate() {
        println!("byte {} = {:08b}", i, b);
    }

    Ok(())
}
