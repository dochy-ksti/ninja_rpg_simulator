use std::path::Path;
use crate::NlResult;
use std::fs::File;
use std::io::Write;
use crate::imp::generate::calc_const_str_src::calc_const_str_src;



pub fn write_generated_src_files<P : AsRef<Path>>(target_dir : P, src : &str) -> NlResult<()>{
    let dir = target_dir.as_ref();
    let mut src_file = File::create(dir.join("generated_src.rs"))?;
    src_file.write_all(src.as_bytes())?;
    let src_text = calc_const_str_src("pub(crate)","GENERATED_SRC_TEXT", src);
    let mut src_txt_file = File::create(dir.join("generated_src_txt.rs"))?;
    src_txt_file.write_all(src_text.as_bytes())?;
    Ok(())
}