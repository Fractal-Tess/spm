#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    process::{Child, Command},
    sync::Mutex,
};
use tauri::{Manager, RunEvent, State};
use tauri_plugin_store::PluginBuilder;

struct TunnelConnection {
    connection: Option<Child>,
}

fn main() {
    let state = Mutex::new(TunnelConnection { connection: None });

    let app = tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![create_tunnel])
        .build(tauri::generate_context!())
        .unwrap();

    app.run(|app_handle, e| match e {
        RunEvent::ExitRequested { api: _, .. } => {
            let connection = app_handle.state::<Mutex<TunnelConnection>>();
            connection
                .lock()
                .unwrap()
                .connection
                .take()
                .unwrap()
                .kill()
                .unwrap();
        }
        _ => {}
    })
}

#[tauri::command]
fn create_tunnel(
    user: String,
    host: String,
    port: String,
    interface: Option<String>,
    connection: State<'_, Mutex<TunnelConnection>>,
) -> (String, bool) {
    let mut tunnel_connection = connection.lock().unwrap();
    match tunnel_connection.connection {
        None => {
            let local_port = portpicker::pick_unused_port().expect("No free ports");
            tunnel_connection.connection = Some(
                Command::new("ssh")
                    .args([
                        "-L",
                        &format!(
                            "{local_port}:{}:{port}",
                            interface.unwrap_or("localhost".to_owned())
                        ),
                        "-N",
                        &format!("{user}@{host}"),
                    ])
                    .spawn()
                    .unwrap(),
            );
            (format!("http://localhost:{local_port}/"), true)
        }
        Some(_) => ("There is already a tunnel".to_owned(), false),
    }
}
