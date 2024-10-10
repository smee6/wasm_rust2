package main

import (
	"encoding/json"
	"net/http"
)

func matrixMultiply(w http.ResponseWriter, r *http.Request) {
	const N = 5000

	// 행렬 할당
	a := make([][]float64, N)
	b := make([][]float64, N)
	c := make([][]float64, N)

	for i := range a {
		a[i] = make([]float64, N)
		b[i] = make([]float64, N)
		c[i] = make([]float64, N)
		for j := range a[i] {
			a[i][j] = 1.0
			b[i][j] = 2.0
		}
	}

	// 행렬 곱셈
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			for k := 0; k < N; k++ {
				c[i][j] += a[i][k] * b[k][j]
			}
		}
	}

	// 결과 행렬의 첫 번째 요소 반환
	response := map[string]float64{"result": c[0][0]}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

func main() {
	// helloworld
	println("Hello, World!")
	http.HandleFunc("/mat", matrixMultiply)
	http.ListenAndServe(":3000", nil)
}
