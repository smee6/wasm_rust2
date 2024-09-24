from fastapi import FastAPI
import numpy as np

app = FastAPI()


@app.get("/matrix_multiply")
def matrix_multiply():
    N = 2500

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
