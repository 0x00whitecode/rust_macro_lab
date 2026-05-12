#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate!(5 + 3), 8);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate!(10 - 4), 6);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate!(3 * 4), 12);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate!(10 / 2), 5);
    }
}