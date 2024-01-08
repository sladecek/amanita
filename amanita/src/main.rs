use amanita_lib::compact_test;
use rand_core::{RngCore, OsRng};

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(compact_test(OsRng));
    }
}