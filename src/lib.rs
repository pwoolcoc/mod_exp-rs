pub fn mod_exp(base: i64, exponent: i64, modules: i64) -> i64 {
    0i64
}

#[cfg(test)] mod tests {
    use super::mod_exp;

    #[test]
    fn test_mod_exp() {
        let base = 4i64;
        let exponent = 13i64;
        let modulus = 497i64;
        assert_eq!(mod_exp(base, exponent, modulus), 445i64);
    }
}
