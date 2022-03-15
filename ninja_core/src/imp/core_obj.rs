use ninja_src::generated_src::test_data::RootIntf;

pub struct CoreObj{
    root : RootIntf
}

impl CoreObj{
    pub fn new(root : RootIntf) -> CoreObj{ CoreObj{ root } }
}