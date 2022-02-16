use std::path::Path;
use crate::NlResult;
use std::fs::File;
use crate::imp::structs::inc_compile_info::IncCompileInfo;
use std::io::Read;

pub(crate) static INC_COMP_FILE_NAME : &str = "incremental_compile_info.json";

pub(crate) fn get_inc_comp_file<P : AsRef<Path>>(target_dir : P) -> NlResult<File>{
    let path = target_dir.as_ref().join(INC_COMP_FILE_NAME);
    Ok(std::fs::File::open(path)?)
}

pub(crate) fn get_inc_info<P : AsRef<Path>>(target_dir : P) -> NlResult<IncCompileInfo>{
    let mut f = match get_inc_comp_file(target_dir){
        Ok(f) => f,
        Err(_) => return Ok(IncCompileInfo::new()),
    };
    let mut s= String::new();
    f.read_to_string(&mut s)?;

    Ok(serde_json::from_str(&s)?)
}

