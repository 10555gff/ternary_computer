use trit_macro::trits;
use ternary_arithmetic::ternary_cpu::trits::Trit4;





struct T80CPU {
    pc: usize,      // byte PC
    mem: Vec<u8>,   // tbin memory
}

struct ROM{

}
struct RAM{

}


fn decodeExecute(byte){

}

fn CPU(){
    let r0;
    let r1;
    let pc=0;
}




//Trit CPU 模拟器终极结构
// struct Register {
//     trits: [Trit4; 8], // 32 trits register
// }




// this.run=fn(){
//     while(true){
//         const byte=read_byte(pc);
//         decodeExecute(byte);
//         pc++;
//     }
// }

// const cpu=new CPU();
// const rom=new ROM();
// const ram=new RAM();


// fn read_byte(address){
//     if(address<0x4000){
//         return rom.read(address);
//     }else{
//         read ram.read(address - 0X4000);
//     }
// }