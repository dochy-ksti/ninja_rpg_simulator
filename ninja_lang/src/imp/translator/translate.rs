use std::path::Path;
use std::fs::{read_dir, File};
use crate::imp::structs::inc_compile_info::IncCompileInfo;
use crate::NlResult;
use std::io::{Read, Write};
use serde_json::{Value, to_string_pretty};
use std::ffi::OsStr;
use crate::imp::translator::get_inc_info::get_inc_info;
use crate::imp::translator::convert_top::convert;

pub(crate) fn translate<P1: AsRef<Path>, P2: AsRef<Path>>(ev_dir : P1, target_dir : P2) -> NlResult<()>{
    let target_dir = target_dir.as_ref();
    let src = read_dir(ev_dir)?;
    let inc_info = get_inc_info(target_dir)?;
    let mut current_inc_info = IncCompileInfo::new();
    for entry in src{
        let entry = entry?;
        let meta = entry.metadata()?;
        let modified_time = meta.modified()?;
        let filename = entry.file_name();
        current_inc_info.add(&filename, modified_time)?;
        if inc_info.contains(&filename, modified_time)? == false{
            let mut file = File::open(entry.path())?;
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            let compiled = convert(&s, filename.to_string_lossy().as_ref())?;
            write_file(&compiled, target_dir, &filename)?;
        }
    }
    Ok(())
}

fn write_file(value : &Value, target_dir : &Path, filename : &OsStr) -> NlResult<()>{
    let path = target_dir.join(filename);
    let s = to_string_pretty(value)?;
    let mut file = File::create(path)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}