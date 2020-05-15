pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2u64.pow(s - 1),
        // slower, more readable
        // 1 | 2 => s as u64,
        // 3..=64 => square(s - 1) * 2,
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
