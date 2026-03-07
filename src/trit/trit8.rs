#[derive(Clone, Copy, Debug)]
pub struct Trit8(u16);

impl Trit8 {
    pub fn new(v: u16) -> Self {
        Trit8(v)
    }
}