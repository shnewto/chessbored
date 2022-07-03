rm -rf dist
trunk build --release
wasm-opt -Oz dist/*.wasm -o dist/*.wasm
trunk serve
# vercel --prod