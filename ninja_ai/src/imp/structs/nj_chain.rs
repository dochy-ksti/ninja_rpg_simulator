
pub struct ChainAnd{
    vec : Vec<u32>,
}

pub struct ChainOr{
    vec : Vec<ChainAnd>
}

pub struct NjChain{
    vec : Vec<ChainOr>
}