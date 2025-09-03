#!/bin/bash

# Build script for Safrimba smart contract
set -e

echo "🔨 Building Safrimba smart contract..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Optimize for production
echo "📦 Building optimized WASM..."
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

# Copy the wasm file to artifacts directory
mkdir -p artifacts
cp target/wasm32-unknown-unknown/release/safrimba.wasm artifacts/

echo "✅ Build complete! WASM file available at artifacts/safrimba.wasm"

# Display file size
echo "📊 Contract size: $(du -h artifacts/safrimba.wasm | cut -f1)"