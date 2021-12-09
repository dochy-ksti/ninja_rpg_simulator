pub(crate) enum RsVal{
    Str(String),
    Arr(RsArray),
    Arr2(RsArray2)
}

pub(crate) struct RsArray{
    vec : Vec<RsItem>
}

pub(crate) struct RsArray2{
    vec : Vec<RsArray>
}

pub(crate) enum RsItem{
    Str(String),
    Int(i64),
    ClosedStr(String),
}