package main

import (
	"encoding/json"
	"math"
	"net/http"
	"time"
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

func countPrimes(limit int) int {
	count := 0
	for num := 2; num <= limit; num++ {
		isPrime := true
		maxDivisor := int(math.Sqrt(float64(num)))
		for i := 2; i <= maxDivisor; i++ {
			if num%i == 0 {
				isPrime = false
				break
			}
		}
		if isPrime {
			count++
		}
	}
	return count
}

func ping(w http.ResponseWriter, r *http.Request) {
	startTime := time.Now()
	limit := 100000
	primeCount := countPrimes(limit)
	duration := time.Since(startTime)

	response := map[string]interface{}{
		"prime_count": primeCount,
		"duration_ms": duration.Milliseconds(),
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

func main() {
	// Hello World 출력
	println("Hello, World!")
	http.HandleFunc("/mat", matrixMultiply)
	http.HandleFunc("/ping", ping)
	http.ListenAndServe(":3000", nil)
}
