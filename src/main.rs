const SHIFT: [u8;4] = [0,2,4,6];
const MASK:  [u8;4] = [0x03,0x0C,0x30,0xC0];

fn set_2bit(word: u8, k: usize, value: u8) -> u8 {
    let mask  = MASK[k];
    let shift = SHIFT[k];
    (word & !mask) | ((value & 0x03) << shift)
}
fn read_2bit(word:u8,k:usize)->u8{
    (word & MASK[k]) >> SHIFT[k]
}
fn clear_2bit(word: u8, k: usize) -> u8 {
    word & !MASK[k]
}
fn toggle_2bit(word: u8, k: usize) -> u8 {
    word ^ MASK[k]
}


fn set2(word: u8, k: u8, value: u8) -> u8 {
    let shift = k * 2;
    let mask = 0x03 << shift;
    // 先将目标位置清零，再存入新值
    (word & !mask) | ((value & 0x03) << shift)
}

fn read2(word: u8, k: u8) -> u8 {
    (word >> (k * 2)) & 0x03
}

fn clear2(word: u8, k: u8) -> u8 {
    let mask = 0x03 << (k * 2);
    word & !mask
}

fn toggle2(word: u8, k: u8) -> u8 {
    let mask = 0x03 << (k * 2);
    word ^ mask
}


// fn set(word:u8,n:u8)->u8{
//     return word | (1 << n)
// }
// fn clear(word:u8,n:u8)->u8{
//     return word & !(1 << n)
// }
// fn toggle(word:u8,n:u8)->u8{
//     return word ^ (1 << n)
// }
// fn read(word:u8,n:u8)->u8{
//     return (word >> n) & 1
// }



fn main() {
    let word:u8=0b11_10_11_01;
    let a=toggle2(word,3);
    println!("result:{:08b}",a);
}