#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) struct Slope{
    slope : u64
}

impl Slope{
    /// panics if distance == 0
    pub(crate) fn new(increase : u32, distance : u32) -> Slope{
        let slope = ((increase as u64) << 32) / (distance as u64);
        Slope{ slope }
    }
}