use std::path::Path;
use crate::imp::translate_ev::translate_ev::translate_ev;
use crate::NlResult;

pub(crate) fn translate_ch<P1: AsRef<Path>, P2: AsRef<Path>>(ch_dir : P1, target_dir : P2) -> NlResult<Vec<String>>{
    translate_ev(ch_dir, target_dir)
}