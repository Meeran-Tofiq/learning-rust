pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again() {
        let result = add(2, 2);
        assert_ne!(result, 99);
    }

    #[test]
    fn it_fails() {
        panic!("at the disco");
    }
}
