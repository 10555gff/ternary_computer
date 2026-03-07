pub trait TritOps: Copy {
    type Output: Copy;
    fn get(&self, n: usize) -> Self::Output;
    fn set(&mut self, n: usize, val: Self::Output);
}


// fn print_first<T: TritOps>(x: T) {
//     println!("{}", x.get(0));
// }
