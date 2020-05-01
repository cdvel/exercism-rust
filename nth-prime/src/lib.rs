pub fn nth(index_nth_prime: usize) -> i64 {
    let mut primes_found = vec![2];

    while primes_found.len() <= index_nth_prime {
        primes_found.push(next_prime(primes_found[primes_found.len() - 1]));
    }

    primes_found[index_nth_prime]
}

pub fn next_prime(n: i64) -> i64 {
    if n <= 1 {
        return 2;
    }

    let mut prime = n;
    let mut found = false;

    while !found {
        prime += 1;
        if is_prime(prime) {
            found = true;
        }
    }
    prime
}

pub fn is_prime(n: i64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ => {
            if n % 2 == 0 || n % 3 == 0 {
                return false;
            }
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}
