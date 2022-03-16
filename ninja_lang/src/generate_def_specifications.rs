
use std::fs::File;
use std::io::Write;
use crate::NlResult;
use crate::imp::generate::calc_const_str_src::calc_const_str_src;

///test用ではなく実用上必要なソース生成
/// defをアップデートしたら走らせるとヨロシ
#[allow(dead_code)]
fn generate_def_specifications() -> NlResult<()>{

    let s = std::fs::read_to_string("ev_def_specifications.json5")?;
    let s = calc_const_str_src("pub","EV_DEF_SPECIFICATIONS", &s);
    let mut file = File::create("src/ev_def_specifications.rs")?;
    file.write_all(s.as_bytes())?;
    let s = std::fs::read_to_string("ch_def_specifications.json5")?;
    let s = calc_const_str_src("pub", "CH_DEF_SPECIFICATIONS", &s);
    let mut file = File::create("src/ch_def_specifications.rs")?;
    file.write_all(s.as_bytes())?;
    let s = std::fs::read_to_string("cv_def_specifications.json5")?;
    let s = calc_const_str_src("pub", "CV_DEF_SPECIFICATIONS", &s);
    let mut file = File::create("src/cv_def_specifications.rs")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

#[test]
fn generate_def_test() -> NlResult<()>{
    generate_def_specifications()
}