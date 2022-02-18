pub(crate) fn calc_const_str_src(accessor : &str, name : &str, s : &str) -> String{
    format!(r###"
{} const {} : &str = r#"{}"#;"###, accessor, name, s)
}