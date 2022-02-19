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
         "src/generated_src",
         crate::generated_src::generated_src::RootIntf::new,
&crate::generated_src::generated_src_txt::GENERATED_SRC_TEXT)?;
    Ok(())
}

fn first() -> NlResult<()>{
    compile_and_write_generated_src("src/json/siyou",
                                    "src/testing/generate_intf_generated",
                                    "src/generated_src")?;
    Ok(())
}