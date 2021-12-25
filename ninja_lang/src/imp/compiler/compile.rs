use std::path::Path;
use std::fs::read_dir;
use crate::imp::compiler::get_inc_info::get_inc_info;
use crate::imp::structs::inc_compile_info::IncCompileInfo;
use crate::NlResult;

pub fn compile<P: AsRef<Path>>(ev_dir : P, target_dir : P) -> NlResult<()>{
    let src = read_dir(ev_dir)?;
    let inc_info = get_inc_info(target_dir)?;
    let mut current_inc_info = IncCompileInfo::new();
    for entry in src{
        let entry = entry?;
        let meta = entry.metadata()?;
        let modified_time = meta.modified()?;
        let filename = entry.file_name();
        current_inc_info.add(&filename, modified_time)?;
        if inc_info.contains(&filename, modified_time) == false{

        }
    }
    Ok(())
}