pub fn factors(mut n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let mut factor = 2;

    while n != 1 {
        if n % factor == 0 {
            result.push(factor);
            n /= factor;
        } else {
            factor += 1;
        }
    }

    result
}
