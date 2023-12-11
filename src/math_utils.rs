/// Greater common diviser
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

/// Least common multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
