#!/bin/bash

ROOT_DIR=$(pwd)

# module directory
MODULES_DIR="$ROOT_DIR/modules"
TARGET_DIR="$ROOT_DIR/wasms"

mkdir -p "$TARGET_DIR"

for dir in "$MODULES_DIR"/*/; do
    if [ -d "$dir" ]; then
        echo "Processing directory: $dir"
        cd "$dir" || { echo "Failed to change directory to $dir"; exit 1; }

        echo "Building module in directory: $dir"
        cargo build --target wasm32-unknown-unknown --release

        if [ $? -eq 0 ]; then
            wasm_dir="$dir/target/wasm32-unknown-unknown/release"
            echo "Searching for wasm files in $wasm_dir..."
            ls "$wasm_dir"

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

        # back to the root directory
        cd "$ROOT_DIR" || { echo "Failed to change directory to $ROOT_DIR"; exit 1; }
    fi
done

echo "Build and copy process completed."
echo "Soon Cargo will run the server."

cargo run --release