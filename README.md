# Setup
1. `cargo install tauri-cli --locked --version ^1.0.0-rc`
2. `cargo install trunk`
3. `cargo install wasm-bindgen-cli` 
4. `rustup target add wasm32-unknown-unknown`

# Running
`cargo tauri dev` for live-reloading dev server, or `cargo tauri build` for a binary.