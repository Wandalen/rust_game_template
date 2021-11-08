#!/usr/bin/env bash
set -e

echo "Compiling..."
cargo build --target=wasm32-unknown-unknown --features wgpu/webgl,winit/web-sys 

echo "Generating bindings..."
mkdir -p target/web
wasm-bindgen --target web --out-dir target/web target/wasm32-unknown-unknown/debug/webgpu.wasm
cp static/index.html target/web/index.html

echo "Serving at http://localhost:8080"
basic-http-server target/web -a 127.0.0.1:8080