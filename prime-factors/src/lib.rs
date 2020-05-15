pub fn factors(n: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let mut candidate = 2;
    let mut remainder = n;
    while remainder > 1 {
        if remainder % candidate == 0 {
            remainder /= candidate;
            divisors.push(candidate);
        } else {
            candidate += 1;
        }
    }
    divisors
}
