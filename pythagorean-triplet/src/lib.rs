pub fn find() -> Option<u32> {
    let nums = f(1000)?;
    Some(nums.0 * nums.1 * nums.2)
}

// find pythagorean numbers {a, b, c} which satisfy a**2 + b**2 == c**2
// and a + b + c == n
// and a < b < c
pub fn f(n: u32) -> Option<(u32, u32, u32)> {
    for a in 1..n {
        for b in a+1..n {
            if a + 2 * b >= n {
                break
            }
            let c = n - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some((a, b, c))
            }
        }
    }
    return None
}
