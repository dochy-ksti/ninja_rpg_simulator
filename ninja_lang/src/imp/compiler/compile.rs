use std::path::Path;
use crate::{cv_def_specifications, NlResult};
use docchi::core::structs::RootObject;
use std::io::Write;

use crate::imp::translate_ch::translate_ch::translate_ch;
use crate::imp::translate_ev::translate_ev::translate_ev;

pub fn compile<P1 : AsRef<Path>, P2 : AsRef<Path>>(src_dir : P1, target_dir : P2) -> NlResult<RootObject>{
    let target_dir = target_dir.as_ref();
    let src_dir = src_dir.as_ref();
    let ev_dir = target_dir.join("ev");
    std::fs::create_dir(&ev_dir).ok();
    translate_ev(src_dir.join("ev"), &ev_dir)?;
    let mut file = std::fs::File::create(target_dir.join("ev.json5"))?;
    file.write_all(crate::ev_def_specifications().as_bytes())?;
    let ch_dir = target_dir.join("ch");
    std::fs::create_dir(&ch_dir).ok();
    translate_ch(src_dir.join("ch"), &ch_dir)?;
    let mut file = std::fs::File::create(target_dir.join("ch.json5"))?;
    file.write_all(crate::ch_def_specifications().as_bytes())?;


    let mut file = std::fs::File::create(target_dir.join("root.json5"))?;
    file.write_all("{}".as_bytes())?;

    let mut file = std::fs::File::create(target_dir.join("cv.json5"))?;
    file.write_all(cv_def_specifications().as_bytes())?;
    Ok(docchi::core::json_dir_to_root(target_dir, true)?)
}