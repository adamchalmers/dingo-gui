I wanted to experiment with Rust GUI apps, so I followed a tutorial on [Tauri and Yew](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe).

![Screenshot of the GUI running](https://raw.githubusercontent.com/adamchalmers/dingo-gui/master/screenshot.png?raw=true)


# Setup
1. `cargo install tauri-cli --locked --version ^1.0.0-rc`
2. `cargo install trunk`
3. `cargo install wasm-bindgen-cli` 
4. `rustup target add wasm32-unknown-unknown`

# Running
`cargo tauri dev` for live-reloading dev server, or `cargo tauri build` for a binary.
