#[cfg(test)]
mod tests {
    use tests_writing::libx::x;
    use tests_writing::liby::y;
    use tests_writing::test_helper::helper;

    #[test]
    fn test_together() {
        assert_eq!(x() + y(), 66);
        assert_eq!(helper(), "Hello");
    }
}
