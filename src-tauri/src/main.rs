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
    connection: Mutex<Option<Child>>,
}
impl TunnelConnection {
    pub fn tunnel(&mut self) -> String {
        let free_local_port = 19999;

        self.connection = Mutex::new(Some(
            Command::new("ssh")
                .args(["-L", "19999:localhost:19999", "-N", "ubuntu@jet-black.xyz"])
                .spawn()
                .unwrap(),
        ));

        free_local_port.to_string()
    }

    pub fn close(&mut self) -> () {
        let _ = self
            .connection
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .kill()
            .unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(TunnelConnection {
            connection: Mutex::new(None),
        })
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
    state: State<'_, TunnelConnection>,
) -> String {
    *state.connection;
    format!("http://localhost:19999/")
}
