#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(my_test::add(1, 2), 3)
    }

    #[test]
    fn test_sub() {
        assert_eq!(my_test::sub(1, 2), -1)
    }

    #[test]
    fn test_mul() {
        assert_eq!(my_test::mul(1, 2), 2)
    }

    #[test]
    fn test_div() {
        assert_eq!(my_test::div(1, 2), 0)
    }
}
