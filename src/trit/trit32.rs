#[derive(Clone, Copy, Debug)]
pub struct Trit32(u64);

impl Trit32 {
    pub fn new(v: u64) -> Self {
        Trit32(v)
    }
}