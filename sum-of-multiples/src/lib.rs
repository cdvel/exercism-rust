pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();

    for f in factors {
        if f > &(0 as u32) {
            multiples.append(
                &mut (1..limit)
                    .filter(|x| x % f == 0 && !multiples.contains(x))
                    .collect::<Vec<u32>>()
            );
        }
    }

    multiples.into_iter().sum()
}
