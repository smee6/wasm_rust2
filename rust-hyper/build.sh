#!/bin/bash

# root directory of this script
ROOT_DIR=$(pwd)

# module directory
MODULES_DIR="$ROOT_DIR/modules"

# set the target directory to copy the generated .wasm files (e.g., wasm folder in the root directory)
TARGET_DIR="$ROOT_DIR/wasms"

# when the target directory does not exist, create it
mkdir -p "$TARGET_DIR"

# visit each subfolder in the modules directory
for dir in "$MODULES_DIR"/*/; do
    if [ -d "$dir" ]; then
        # change directory to the module directory
        echo "Processing directory: $dir"
        cd "$dir" || { echo "Failed to change directory to $dir"; exit 1; }

        # run cargo build in the subfolder (wasm target)
        echo "Building module in directory: $dir"
        cargo build --target wasm32-unknown-unknown --release

        # check if the build is successful or not
        if [ $? -eq 0 ]; then
            # copy the wasm file to the target directory
            wasm_dir="$dir/target/wasm32-unknown-unknown/release"
            echo "Searching for wasm files in $wasm_dir..."
            ls "$wasm_dir"

            # search for the wasm file in the release directory
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

# log the completion of the build and copy process
echo "Build and copy process completed."
echo "Soon Cargo will run the server."

# run the server
cargo run --release