pub fn x() -> u64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        assert_eq!(x(), 42u64);
    }
}
