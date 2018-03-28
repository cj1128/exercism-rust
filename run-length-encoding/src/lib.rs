pub fn encode(s: &str) -> String {
    let mut result = String::new();
    let mut count = 1;
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.next() {
        if iter.peek() == Some(&&c) {
            count += 1;
            continue
        }
        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(c);
        count = 1;
    }

    result
}

pub fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut digit = String::new();

    for c in s.chars() {
        if c.is_numeric() {
            digit.push(c);
        } else {
            let count = digit.parse().unwrap_or(1);
            let text = std::iter::repeat(c).take(count).collect::<String>();
            result.push_str(&text);
            digit.clear();
        }
    }

    result
}
