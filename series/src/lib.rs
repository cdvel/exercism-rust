pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    if let Some(i) = digits.len().checked_sub(len) {
        for x in 0..=i {
            let end = x + len;
            result.push(digits[x..end].to_string());
        }
    }
    result
}
