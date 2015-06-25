//! Modular Exponentiation
//!
//! Implementation of a modular exponentiation algorithm based on the algorithm from
//! 'Applied Cryptography'. You can find more details in that book, or on
//! [wikipedia](https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method).

extern crate num;

use std::ops::{Shr};
use num::traits::{Num, One, Zero, Bounded};

#[allow(non_snake_case)]
/// Performs the exponentiation
///
/// All parameters are generic, provided they implement the following traits:
///
/// * Num
/// * PartialOrd
/// * Shr<T, Output=T>
/// * Copy
/// * Bounded
///
/// You can find the `Num` and `Bounded` traits in the [num](https://crates.io/crate/num) crate.
///
/// # Examples
///
/// ```
/// use mod_exp::mod_exp;
///
/// assert_eq!(mod_exp(5, 3, 13), 8);
/// ```
///
/// # Panics
///
/// The function does an `assert!` to verify that the data type of `base` is
/// large enough that the result won't overflow during the computation
pub fn mod_exp<T>(base: T, exponent: T, modulus: T) -> T where T: Num + PartialOrd + Shr<T, Output=T> + Copy + Bounded {
    let ONE: T = One::one();
    let TWO: T = ONE + ONE;
    let ZERO: T = Zero::zero();
    let MAX: T = Bounded::max_value();

    assert!((modulus - ONE) * (modulus - ONE) < MAX);

    let mut result = ONE;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= ZERO {
            break;
        }

        if exponent % TWO == ONE {
            result = (result * base) % modulus;
        }

        exponent = exponent >> ONE;
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
