#!/bin/bash
echo "Running node.js fibonacci sequence.."
jsResult=$(node js/index.js)
echo "Running go fibonacci sequence.."
goResult=$(go run go/main.go)
echo "Running rust fibonacci sequence.."
rustResult=$(./rust/target/release/rust)
echo "Finished"
echo "Benchmark: "
echo "Node.js: $jsResult" 
echo "Go: $goResult" 
echo "Rust: $rustResult"