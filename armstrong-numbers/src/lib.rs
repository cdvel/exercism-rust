pub fn is_armstrong_number(num: u32) -> bool {
    let n_digits = ((num as f32).log(10.0) + 1.0) as u32;
    let mut remainder = num;
    let mut sum = 0;

    while remainder > 0 {
        sum += (remainder % 10).pow(n_digits);
        remainder /= 10;
    }

    sum == num
}
