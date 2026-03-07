pub trait TritOps {
    type Ttype: Copy;
    fn get(&self, n: usize) -> Self::Ttype;
    fn set(&mut self, n: usize, val: Self::Ttype);
}


// fn print_first<T: TritOps>(x: T) {
//     println!("{}", x.get(0));
// }
