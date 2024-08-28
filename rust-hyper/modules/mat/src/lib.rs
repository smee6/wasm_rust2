use wasm_bindgen::prelude::*;

#[no_mangle]
pub extern "C" fn matrix_multiply() -> f64 {
    const N: usize = 1000;
    let a = [[1.0; N]; N];
    let b = [[2.0; N]; N];
    let mut c = [[0.0; N]; N];

    // 행렬 곱셈 수행
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    // 결과로 행렬 c의 첫번째 원소를 반환
    c[0][0]
}
