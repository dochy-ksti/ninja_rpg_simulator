use std::path::Path;
use std::fs::read_dir;
use crate::NlResult;

pub fn compile<P: AsRef<Path>>(ev_dir : P, target_dir : P) -> NlResult<()>{
    let src = read_dir(ev_dir)?;
    for entry in src{
        let entry = entry?;
        let meta = entry.metadata()?;
        let modified_time = meta.modified()?;
    }
    Ok(())
}