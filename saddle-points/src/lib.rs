pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let c = input[row][col];
            if input[row].iter().all(|&n| n <= c) && input.iter().all(|row| row[col] >= c) {
                result.push((row, col));
            }
        }
    }

    result
}
