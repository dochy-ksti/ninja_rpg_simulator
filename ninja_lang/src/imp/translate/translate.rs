use std::collections::HashMap;
use std::path::Path;
use std::fs::{read_dir, File};
use crate::imp::structs::inc_compile_info::IncCompileInfo;
use crate::NlResult;
use std::io::{Read, Write};
use serde_json::{Value, to_string_pretty};
use std::ffi::OsStr;
use crate::imp::convert::convert_top::convert;
use crate::imp::convert::get_inc_info::get_inc_info;
use crate::imp::structs::param_name_map::ParamNameMap;

pub(crate) fn translate_ev<P1: AsRef<Path>, P2: AsRef<Path>>(ev_dir : P1, target_dir : P2) -> NlResult<()> {
    translate(ev_dir.as_ref(), target_dir.as_ref(), &ev_map())
}
pub(crate) fn translate_ch<P1: AsRef<Path>, P2: AsRef<Path>>(ch_dir : P1, target_dir : P2) -> NlResult<()> {
    translate(ch_dir.as_ref(), target_dir.as_ref(), &ch_map())
}

pub(crate) fn translate(src_dir : &Path, target_dir : &Path, names : &ParamNameMap) -> NlResult<()> {
    let src = read_dir(src_dir)?;
    let inc_info = get_inc_info(target_dir)?;
    let mut current_inc_info = IncCompileInfo::new();
    for entry in src {
        let entry = entry?;
        let meta = entry.metadata()?;
        let modified_time = meta.modified()?;
        let filename = entry.file_name();
        current_inc_info.add(&filename, modified_time)?;
        if inc_info.contains(&filename, modified_time)? == false {
            let mut file = File::open(entry.path())?;
            let mut s = String::new();
            file.read_to_string(&mut s)?;

            let compiled = convert(&s, filename.to_string_lossy().as_ref(), names)?;
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

fn ev_map() -> ParamNameMap{
    let mut map = ParamNameMap::new();
    map.add("value", "v");
    map.add("seq", "eseq");
    map.add("v", "ev");
    map.add("c", "ec");
    map
}

fn ch_map() -> ParamNameMap{
    let mut map = ParamNameMap::new();
    map.add("value", "o");
    map.add("seq", "cseq");
    map.add("v", "cv");
    map.add("c", "cc");
    map
}