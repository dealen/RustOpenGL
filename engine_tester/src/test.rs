#[cfg(test)]
mod tests {

    #[test]
    fn test_check_if_working() {
        let result = kubol_core::check_if_working();
        assert_eq!(result, "Hello, world! from kubol_core lib.");
    }
}
