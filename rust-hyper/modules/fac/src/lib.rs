#[no_mangle]
pub extern "C" fn factorize(n: u64) -> u64 {
    let mut d = 2;
    let mut num = n;
    let mut factors = 0;

    while num > 1 {
        while num % d == 0 {
            factors += 1;
            num /= d;
        }
        d += 1;
        if d * d > num {
            if num > 1 {
                factors += 1;
            }
            break;
        }
    }
    factors
}
