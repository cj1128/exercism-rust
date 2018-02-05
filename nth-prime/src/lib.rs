use std::f32;

pub fn nth(mut i: usize) -> Result<usize, &'static str> {
    if i <= 0 {
        return Err("invalid input");
    }

    if i == 1 {
        return Ok(2);
    }

    i -= 1;

    let mut n = 1;

    while i > 0 {
        n += 2;
        if is_prime(n) {
            i -= 1;
        }
    }

    Ok(n)
}

fn is_prime(n: usize) -> bool {
    let bound = (n as f32).sqrt().floor() as usize + 1;
    for i in 2..bound {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
