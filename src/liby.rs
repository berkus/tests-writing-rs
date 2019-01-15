pub fn y() -> u64 {
    24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_y() {
        assert_eq!(y(), 24u64);
    }
}
