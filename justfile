# Prints UTC time and user configured timezone time
time:
  @cargo build -p ostime --target wasm32-wasi
  @cargo build -p ostime
  @cargo build -p ostime --target x86_64-pc-windows-gnu
  time ./target/debug/ostime
  time wine ./target/x86_64-pc-windows-gnu/debug/ostime.exe
  time wasmer target/wasm32-wasi/debug/ostime.wasm
  time wasmtime target/wasm32-wasi/debug/ostime.wasm
  time wasmedge target/wasm32-wasi/debug/ostime.wasm

# Doesn't run
http:
  @cargo build -p http-wasmedge
  @cargo build -p http-wasmedge --target wasm32-wasi
  time wasmedge target/wasm32-wasi/debug/http.wasm

# 
