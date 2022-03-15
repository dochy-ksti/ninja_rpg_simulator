use ninja_lang::{NlResult, make_intf, compile_and_write_generated_src};

#[test]
fn generate_test_data() -> NlResult<()>{
    //first()?;
    second()?;



    Ok(())
}

pub fn second() -> NlResult<()>{
    make_intf("src/json/test_data",
               "src/testing/test_data_converted",
         "src/generated_src/test_data.rs",
         crate::generated_src::test_data::RootIntf::new)?;
    Ok(())
}

fn first() -> NlResult<()>{
    compile_and_write_generated_src("src/json/test_data",
                                    "src/testing/test_data_converted",
                                    "src/generated_src/test_data.rs")?.ok();
    Ok(())
}