# Dockerfile

# Python 베이스 이미지 선택
FROM python:3.9-slim

# 작업 디렉토리 설정
WORKDIR /app

# FastAPI, Uvicorn, numpy 설치
RUN pip install --no-cache-dir fastapi uvicorn numpy

# 애플리케이션 소스 코드 복사
COPY . .

# uvicorn 서버 실행
CMD ["uvicorn", "server:app", "--reload", "--host", "0.0.0.0", "--port", "3000"]
