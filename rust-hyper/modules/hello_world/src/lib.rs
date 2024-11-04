#[no_mangle]
pub extern "C" fn hello_world() -> i32 {
    let mut count = 0;
    let limit: u32 = 1000000;

    for num in 2..=limit {
        let mut is_prime = true;
        let max_divisor = (num as f64).sqrt() as u32;

        for i in 2..=max_divisor {
            if num % i == 0 {
                is_prime = false;
                break;
            } 
        }
        if is_prime {
            count += 1;
        }
    }
    
    count
}
