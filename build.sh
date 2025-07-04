#!/bin/bash

# Script pour lancer fmt, clippy et run en séquence

echo "🔧 Formatting code with cargo fmt..."
cargo fmt

if [ $? -ne 0 ]; then
    echo "❌ cargo fmt failed"
    exit 1
fi

echo "🔍 Running clippy..."
cargo clippy

if [ $? -ne 0 ]; then
    echo "❌ cargo clippy failed"
    exit 1
fi

echo "🚀 Running the application..."
cargo run 