use std::path::Path;
use crate::NlResult;
use std::fs::File;
use std::io::Write;

pub(crate) fn write_generated_src_files<P : AsRef<Path>>(target_dir : P, src : &str) -> NlResult<()>{
    let dir = target_dir.as_ref();
    let mut src_file = File::create(dir.join("generated_src.rs"))?;
    src_file.write_all(src.as_bytes())?;
    let mut src_text = format!(r###"
pub(crate) fn generate_src_text() -> &'static str{{
    r#"{}"#
}}
"###, src);
    let mut src_txt_file = File::create(dir.join("generated_src_txt.rs"))?;
    src_txt_file.write_all(src_text.as_bytes())?;
    Ok(())
}