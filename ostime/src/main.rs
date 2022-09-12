// Depends on wasm-bindgen
use chrono::prelude::*;

fn main() {
    // Works flawlessly on: wasmer, wasmtime
    // Doesn't work on: wasm3
    println!("{}", Utc::now());
    // This doesn't get the local timezone,
    // prints +00.00 regardless the system timezone
    println!("{}", Local::now());
}
