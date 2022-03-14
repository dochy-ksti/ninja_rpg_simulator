use std::path::Path;
use crate::NlResult;
use std::fs::File;
use std::io::Write;
use crate::imp::generate::calc_const_str_src::calc_const_str_src;


/// returns true when the files are changed
pub fn write_generated_src_file<P : AsRef<Path>>(target_file_path : P, src : &str) -> NlResult<bool>{
    let file_path = target_file_path.as_ref();
    
    if let Ok(content) = std::fs::read_to_string(&file_path){
        if src == content{
            return Ok(false);
        }
    }
    
    let mut src_file = File::create(&file_path)?;
    src_file.write_all(src.as_bytes())?;
    Ok(true)
}