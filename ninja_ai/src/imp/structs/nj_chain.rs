pub struct ChainAnd{
    vec : Vec<String>,
}

pub struct ChainOr{
    vec : Vec<ChainAnd>
}

pub struct NjChain{
    vec : Vec<ChainOr>
}