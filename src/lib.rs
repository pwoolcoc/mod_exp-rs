extern crate num;

pub fn mod_exp(base: i64, exponent: i64, modulus: i64) -> i64 {
    assert!((modulus - 1) * (modulus - 1) < std::i64::MAX);

    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= 0 {
            break;
        }

        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    result
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
