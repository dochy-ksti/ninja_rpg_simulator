use std::path::Path;
use crate::NlResult;
use docchi::core::structs::RootObject;
use crate::imp::translator::translate::translate;
use std::io::Write;

pub fn compile<P1 : AsRef<Path>, P2 : AsRef<Path>>(src_dir : P1, target_dir : P2) -> NlResult<RootObject>{
    let target_dir = target_dir.as_ref();
    translate(src_dir, target_dir)?;
    let target_dir = std::fs::canonicalize(target_dir)?;
    let parent = target_dir.parent().ok_or_else(|| format!("Target_dir doesn't have its parent {:?}", target_dir))?;
    let mut file = std::fs::File::create(parent.join("event.json5"))?;
    file.write_all(crate::ev_def_specifications().as_bytes())?;
    let mut file = std::fs::File::create(parent.join("root.json5"))?;
    file.write_all("{}".as_bytes())?;
    Ok(docchi::core::json_dir_to_root(parent, true)?)
}