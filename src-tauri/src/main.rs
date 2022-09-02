#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    process::{Child, Command},
    sync::Mutex,
};
use tauri::State;
use tauri_plugin_store::PluginBuilder;

struct TunnelConnection {
    connection: Option<Child>,
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(Mutex::new(TunnelConnection { connection: None }))
        .invoke_handler(tauri::generate_handler![tunnel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn tunnel(
    user: String,
    host: String,
    port: String,
    interface: Option<String>,
    state: State<'_, Mutex<TunnelConnection>>,
) -> String {
    let local_port = portpicker::pick_unused_port().expect("No free ports");

    state.lock().unwrap().connection = Some(
        Command::new("ssh")
            .args([
                "-L",
                &format!(
                    "{local_port}:{}:{port}",
                    interface.unwrap_or("localhost".to_string())
                ),
                "-N",
                &format!("{user}@{host}"),
            ])
            .spawn()
            .unwrap(),
    );

    format!("http://localhost:{local_port}/")
}
