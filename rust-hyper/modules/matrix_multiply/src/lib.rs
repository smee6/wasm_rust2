#[no_mangle]
pub extern "C" fn matrix_multiply() -> f64 {
    const N: usize = 100;
    
    // Use heap-allocated matrices
    let a = vec![vec![1.0; N]; N];
    let b = vec![vec![2.0; N]; N];
    let mut c = vec![vec![0.0; N]; N];

    // Matrix multiplication
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    // Return the first element of the result matrix
    c[0][0]
}
