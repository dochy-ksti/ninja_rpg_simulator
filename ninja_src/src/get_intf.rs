use ninja_lang::{make_intf, NlResult};
use crate::generated_src::test_data::RootIntf;

pub fn get_intf() -> NlResult<RootIntf>{
    Ok(make_intf("src/json/test_data",
              "src/testing/test_data_converted",
              "src/generated_src/test_data.rs",
              crate::generated_src::test_data::RootIntf::new)?)
}