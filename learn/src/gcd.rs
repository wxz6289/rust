pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "Inputs must be positive integers");
    while m != 0 {
        if m < n {
            let tmp = m;
            m = n;
            n = tmp;
        }
        m = m % n;
    }
    n
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(56, 98), 14);
        assert_eq!(gcd(100, 25), 25);
    }

    #[should_panic(expected = "Inputs must be positive integers")]
    #[test]
    fn test_gcd_zero_input() {
        gcd(0, 5);
    }
    #[should_panic(expected = "Inputs must be positive integers")]
    #[test]
    fn test_gcd_negative_input() {
        gcd(5, 0);
    }
    #[should_panic(expected = "Inputs must be positive integers")]
    #[test]
    fn test_gcd_both_zero() {
        gcd(0, 0);
    }
}
