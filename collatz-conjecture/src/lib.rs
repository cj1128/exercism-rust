// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("must provide positive integer");
    }

    let mut i = 0;
    while n != 1 {
        i += 1;
        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        }
    }

    Ok(i)
}
