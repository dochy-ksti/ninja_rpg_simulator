#![deny(unreachable_pub)]
#![deny(unused_crate_dependencies)]

mod goals;
mod kumamoto;
mod imp;

#[test]
fn test(){
    assert_eq!(2 + 2, 4);
}