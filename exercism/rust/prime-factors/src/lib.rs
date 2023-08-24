pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut candidate = 2;
    let mut candidates = (3..).step_by(2);

    while n > 1 {
        if n % candidate == 0 {
            n /= candidate;
            factors.push(candidate);
        } else {
            candidate = candidates.next().unwrap();
        }
    }

    factors
}
