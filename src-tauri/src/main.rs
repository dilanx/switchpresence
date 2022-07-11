#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use discord_rpc_client::Client;
use discord_rpc_client::models::rich_presence::Activity;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;
use std::sync::Mutex;

struct App {
  client: Mutex<Client>,
}

fn main() {
  tauri::Builder::default()
    .manage(App {
      client: Mutex::new(init_discord_client()),
    })
    .invoke_handler(tauri::generate_handler![update_presence])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init_discord_client() -> Client {
  let mut client = Client::new(995819278672601099);
  client.start();
  return client;
}

#[tauri::command]
fn update_presence(game: String, state: State<App>) {
  let activity = Activity::new()
    .state(&game)
    .assets(|assets| {
      assets
        .large_image("switch")
        .large_text(game)
        .small_image("green")
        .small_text("SwitchPresence")
    })
    .timestamps(|ts| {
      let time = SystemTime::now();
      let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();
      ts.start(since_the_epoch)
    });

  let mut client = state.client.lock().unwrap();
  let _ = (*client).set_activity(|_| activity);
}