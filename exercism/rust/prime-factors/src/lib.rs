pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut i = 2;

    while i <= n {
        if n % i == 0 {
            n /= i;
            factors.push(i);
        }
        else {
            i += 1;
        }
    }

    factors
}
