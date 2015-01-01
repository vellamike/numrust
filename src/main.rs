fn add_one(x: int) -> int {
    x + 1i
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    #[test]
    fn it_works() {
    }

    #[test]
    fn test_adds_one() {
        assert_eq!(3i, super::add_one(2i));
    }

}
