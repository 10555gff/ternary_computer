#[derive(Clone, Copy, Debug)]
pub struct Trit16(u32);

impl Trit16 {
    pub fn new(v: u32) -> Self {
        Trit16(v)
    }
}