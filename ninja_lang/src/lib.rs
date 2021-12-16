#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

mod error;
mod imp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
