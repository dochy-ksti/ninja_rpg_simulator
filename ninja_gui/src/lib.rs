#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

use crate::imp::start_loop::start_loop;

mod imp;
#[cfg(test)]#[allow(dead_code)]
mod testing;

pub fn test_window(){
    start_loop()
}
