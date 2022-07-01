use std::num::NonZeroU32;

struct Tes{
    item : Option<Inner>,
}

enum Huga{
    NoData,
    Null,
    Inner(Inner)
}

struct Inner{
    hoge : NonZeroU32,
    //hoge : u32,
    huga : Vec<u64>
    //a1 : u32,
    //a2 : u32,
    //a3 : u32,
}


#[test]
fn test_something(){
    //println!("size {}", size_of::<Huga>());
    //println!("inner size {}", size_of::<Inner>());
}