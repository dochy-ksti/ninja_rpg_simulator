use std::fs::File;
use std::io::Write;
use crate::calc_const_str_src::calc_const_str_src;
use crate::NlResult;

#[test]
fn generate_def_specifications() -> NlResult<()>{

    let s = std::fs::read_to_string("ev_def_specifications.json5")?;
    let s = calc_const_str_src("ev_def_specifications", &s);
    let mut file = File::create("src/ev_def_specifications.rs")?;
    file.write_all(s.as_bytes());
    let s = std::fs::read_to_string("ch_def_specifications.json5")?;
    let s = calc_const_str_src("ch_def_specifications", &s);
    let mut file = File::create("src/ch_def_specifications.rs")?;
    file.write_all(s.as_bytes());
    let s = std::fs::read_to_string("cv_def_specifications.json5")?;
    let s = calc_const_str_src("cv_def_specifications", &s);
    let mut file = File::create("src/cv_def_specifications.rs")?;
    file.write_all(s.as_bytes());
    Ok(())
}