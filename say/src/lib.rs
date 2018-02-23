const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 8] = [
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

const HUNDREDS: [(u64, &str); 7] = [
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
    (100, "hundred"),
];

pub fn encode(n: u64) -> String {
    match n {
        0 ... 19 => ONES[n as usize].to_string(),
        20 ... 99 => {
            let major = n / 10 - 2;
            let minor = n % 10;
            let result = TENS[major as usize].to_string();

            if minor == 0 {
                result
            } else {
                format!("{}-{}", result, ONES[minor as usize])
            }
        }
        _ => {
            let (i, b) = HUNDREDS
                .iter()
                .map(|x| x.0)
                .enumerate()
                .find(|x| n >= x.1)
                .unwrap();
            let major = n / b;
            let minor = n % b;
            let result = format!("{} {}", encode(major), HUNDREDS[i].1);

            if minor == 0 {
                result
            } else {
                format!("{} {}", result, encode(minor))
            }
        }
    }
}
