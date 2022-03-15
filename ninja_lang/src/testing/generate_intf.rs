use crate::{NlResult, make_intf, compile_and_write_generated_src};

#[test]
fn generate_intf() -> NlResult<()>{
    //first()?;
    second()?;



    Ok(())
}

fn second() -> NlResult<()>{
    make_intf("src/json/siyou",
               "src/testing/generate_intf_generated",
         "src/generated_src/siyou.rs",
         crate::generated_src::siyou::RootIntf::new)?;
    Ok(())
}

fn first() -> NlResult<()>{
    compile_and_write_generated_src("src/json/siyou",
                                    "src/testing/generate_intf_generated",
                                    "src/generated_src/siyou.rs")?.ok();
    Ok(())
}