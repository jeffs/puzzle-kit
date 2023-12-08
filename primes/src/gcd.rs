/// Returns the Greatest Common Divisor of a and b.  gcd(0, 0) is not defined.
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    assert!(a != 0 || b != 0);
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

/// Returns the Least Common Multiple of a and b.  Panics if eíther a or b is 0.
pub fn lcm(a: usize, b: usize) -> usize {
    assert!(a != 0 && b != 0);
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_basics() {
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    fn lcm_basics() {
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 2), 2);
        assert_eq!(lcm(2, 1), 2);
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(6, 4), 12);
    }
}
