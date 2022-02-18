use ninja_lang::{NlResult, make_intf, compile, compile_and_write_generated_src};

#[test]
fn generate_intf() -> NlResult<()>{
    //first()?;

    make_intf("src/json/siyou",
               "src/testing/generate_intf_generated",
         "src/generated_src",
         crate::generated_src::generated_src::RootIntf::new,
&crate::generated_src::generated_src_txt::generated_src_text);

    Ok(())
}

fn first() -> NlResult<()>{
    compile_and_write_generated_src("src/json/siyou",
                                    "src/testing/generate_intf_generated",
                                    "src/generated_src")?;
    Ok(())
}