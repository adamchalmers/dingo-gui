#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dns;

#[tauri::command]
fn resolve(hostname: &str) -> Result<String, String> {
    let ips = dns::resolve(hostname).map_err(|e| e.to_string())?;
    let out = ips.into_iter().collect::<Vec<_>>().join(", ");
    Ok(out)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![resolve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
