#!/bin/bash

set -e

# Define output directory
BUILD_DIR="dist"
WASM_FILE=$(find "$BUILD_DIR" -name "*.wasm")

# Step 1: Build the project using trunk
echo "Building the Leptos CSR project with Trunk..."
trunk build --release

# Step 2: Check if the build succeeded
if [ -z "$WASM_FILE" ]; then
    echo "Error: WASM file not found in the build directory."
    exit 1
fi

echo "WASM file found: $WASM_FILE"

# Step 3: Optimize the WASM file for size and remove unused functions
echo "Optimizing the WASM file for size and removing unused functions with wasm-opt..."
wasm-opt -Oz --dce -o "$WASM_FILE" "$WASM_FILE"

# Step 4: Further optimize the WASM file for speed
echo "Optimizing the WASM file for speed with wasm-opt..."
wasm-opt -O3 -o "$WASM_FILE" "$WASM_FILE"

echo "Optimization complete. Optimized WASM file: $WASM_FILE"

# Step 5: Optional - Print the final size of the optimized WASM
echo "Final WASM file size:"
ls -lh "$WASM_FILE"

echo "Build and optimization process completed successfully."
