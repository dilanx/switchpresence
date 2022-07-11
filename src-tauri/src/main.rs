#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use discord_rpc_client::Client;
use discord_rpc_client::models::rich_presence::Activity;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{State, CustomMenuItem, Menu, MenuItem, Submenu, AboutMetadata};
use std::sync::Mutex;

struct App {
    client: Mutex<Client>,
}

fn main() {
    tauri::Builder::default()
        .manage(App {
            client: Mutex::new(init_discord_client()),
        })
        .invoke_handler(tauri::generate_handler![update_presence, clear_presence])
        .menu(init_window_menu("SwitchPresence".to_string()))
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "sp-github" => {
                    open::that("https://github.com/dilanx/switchpresence").unwrap();
                }
                "sp-clear" => {
                    event.window().emit("event_clear_presence", {})
                        .expect("failed to emit clear event");
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_discord_client() -> Client {
    let mut client = Client::new(995819278672601099);
    client.start();
    return client;
}

fn init_window_menu(title: String) -> Menu {
    let mut menu = Menu::new();
    if cfg!(target_os = "macos") {
        let about = Submenu::new(
            title,
            Menu::new()
                .add_native_item(
                    MenuItem::About(
                        "SwitchPresence".to_string(),
                        AboutMetadata::new().copyright("Copyright © 2022 Dilan Nair (dilanxd.com)".to_string())
                    )
                )
                .add_item(CustomMenuItem::new("sp-github", "View on GitHub"))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit)
        );
        menu = menu.add_submenu(about);
    }
    let discord = Submenu::new(
        "Discord",
        Menu::new()
            .add_item(CustomMenuItem::new("sp-clear", "Clear Rich Presence").accelerator("CmdOrCtrl+K"))
    );

    return menu.add_submenu(discord);
    
}

#[tauri::command(async)]
fn update_presence(game: String, state: State<App>) -> bool {
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
    let act = (*client).set_activity(|_| activity);
    return act.is_ok();
}

#[tauri::command(async)]
fn clear_presence(state: State<App>) -> bool {
    let mut client = state.client.lock().unwrap();
    return (*client).clear_activity().is_ok();
}