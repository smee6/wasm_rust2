#!/bin/bash

# 루트 디렉토리 경로 설정
ROOT_DIR=$(pwd)

# 모듈 디렉토리 경로 설정
MODULES_DIR="$ROOT_DIR/modules"

# 생성된 .wasm 파일을 복사할 경로 설정 (예: root 디렉토리의 wasm 폴더)
TARGET_DIR="$ROOT_DIR"

# 복사할 디렉토리가 없다면 생성
mkdir -p "$TARGET_DIR"

# modules 디렉토리 내의 각 서브 폴더를 순회
for dir in "$MODULES_DIR"/*/; do
    if [ -d "$dir" ]; then
        # 서브 폴더로 이동
        echo "Processing directory: $dir"
        cd "$dir" || { echo "Failed to change directory to $dir"; exit 1; }

        # 서브 폴더 내에서 cargo build 실행 (wasm 타겟)
        echo "Building module in directory: $dir"
        cargo build --target wasm32-unknown-unknown --release

        # 빌드가 성공했는지 확인
        if [ $? -eq 0 ]; then
            # 릴리즈 폴더에서 wasm 파일 복사
            wasm_dir="$dir/target/wasm32-unknown-unknown/release"
            echo "Searching for wasm files in $wasm_dir..."
            ls "$wasm_dir"

            # find 명령어로 wasm 파일 찾기
            wasm_file=$(find "$wasm_dir" -name "*.wasm" | head -n 1)

            if [ -n "$wasm_file" ]; then
                echo "Found wasm file: $wasm_file"
                echo "Copying $wasm_file to $TARGET_DIR"
                cp "$wasm_file" "$TARGET_DIR/"
            else
                echo "No wasm file found in $wasm_dir"
            fi
        else
            echo "Build failed in $dir"
        fi

        # 루트 디렉토리로 돌아감
        cd "$ROOT_DIR" || { echo "Failed to change directory to $ROOT_DIR"; exit 1; }
    fi
done

echo "Build and copy process completed."

cargo run --release

echo "Build, copy, and run process completed."