use amanita_lib;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use amanita_lib::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}