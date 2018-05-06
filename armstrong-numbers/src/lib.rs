pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let length = digits.len();
    return digits.iter().fold(0, |acc, x| acc + x.pow(length as u32)) == num
}
