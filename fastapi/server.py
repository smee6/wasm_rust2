from fastapi import FastAPI
import numpy as np
import math


def count_primes(limit):
    count = 0

    for num in range(2, limit + 1):
        is_prime = True
        max_divisor = int(math.sqrt(num))

        for i in range(2, max_divisor + 1):
            if num % i == 0:
                is_prime = False
                break

        if is_prime:
            count += 1

    return count

app = FastAPI()


@app.get("/mat")
def matrix_multiply():
    N = 5000

    # 행렬 할당
    a = [[1.0 for _ in range(N)] for _ in range(N)]
    b = [[2.0 for _ in range(N)] for _ in range(N)]
    c = [[0.0 for _ in range(N)] for _ in range(N)]

    # 행렬 곱셈
    for i in range(N):
        for j in range(N):
            for k in range(N):
                c[i][j] += a[i][k] * b[k][j]

    # 결과 행렬의 첫 번째 요소 반환
    return {"result": c[0][0]}

@app.get("/ping")
def read_root():
    a = count_primes(1000000)
    
    return {"Hello": "World", "primes": a}