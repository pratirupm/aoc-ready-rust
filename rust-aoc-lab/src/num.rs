//! Small number-theory helpers.

/// The greatest common divisor of two numbers.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

/// The least common multiple of two numbers.
///
/// Dividing before multiplying keeps the intermediate value small.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

/// The least common multiple of a list of numbers.
pub fn lcm_many(values: &[u64]) -> u64 {
    values.iter().fold(1, |total, &value| lcm(total, value))
}

/// `base` raised to `exponent`, modulo `modulus`.
pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    base %= modulus;
    let mut result: u64 = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exponent /= 2;
    }

    result
}

/// The non-negative remainder, even when `value` is negative.
pub fn rem_euclid(value: i64, modulus: i64) -> i64 {
    value.rem_euclid(modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_coprime_numbers_is_one() {
        assert_eq!(gcd(9, 28), 1);
    }

    #[test]
    fn gcd_of_shared_factors() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn lcm_of_coprime_numbers() {
        assert_eq!(lcm(4, 9), 36);
    }

    #[test]
    fn lcm_of_many_numbers() {
        assert_eq!(lcm_many(&[3, 5, 7]), 105);
    }

    #[test]
    fn mod_pow_matches_repeated_multiplication() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
    }

    #[test]
    fn negative_remainders_become_positive() {
        let value = -3_i64;
        assert_eq!(value % 7, -3);
        assert_eq!(rem_euclid(value, 7), 4);
    }
}
