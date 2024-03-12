pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;
    let candidates = std::iter::once(2).chain((3..=n).step_by(2));

    for candidate in candidates {
        while n % candidate == 0 {
            n /= candidate;
            factors.push(candidate);
        }
        if n == 1 {
            return factors;
        }
    }
    return factors;
}
