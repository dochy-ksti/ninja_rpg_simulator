pub(crate) fn calc_const_str_src(name : &str, s : &str) -> String{
    format!(r#"
pub(crate) const {} : &str = "{}";"#, name, s)
}