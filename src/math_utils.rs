/// Greater common diviser
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
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
