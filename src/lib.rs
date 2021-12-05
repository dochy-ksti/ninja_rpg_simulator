#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

mod goals;
mod kumamoto;
mod imp;
mod error;

#[test]
fn test(){
    assert_eq!(2 + 2, 4);
}