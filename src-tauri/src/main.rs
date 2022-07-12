#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use discord_rpc_client::Client;
use discord_rpc_client::models::rich_presence::Activity;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use tauri::{
    State,
    CustomMenuItem,
    Menu,
    MenuItem,
    Submenu,
    AboutMetadata,
    SystemTray,
    SystemTrayMenu,
    SystemTrayEvent,
    SystemTrayMenuItem,
    Manager
};


struct App {
    client: Mutex<Client>,
    client_win: Mutex<DiscordIpcClient>,
}

fn main() {
    tauri::Builder::default()
        .manage(App {
            client: Mutex::new(init_discord_client()),
            client_win: Mutex::new(init_discord_client_win()),
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
        .system_tray(init_system_tray())
        .on_system_tray_event(|app, event| match event {
            #[cfg(target_os = "windows")]
            SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
                app.get_window("main").unwrap()
                    .set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick {id, ..} => {
                match id.as_str() {
                    "sp-focus" => {
                        app.get_window("main").unwrap()
                            .set_focus().unwrap();
                    } 
                    "sp-edit" => {
                        app.get_window("main").unwrap()
                            .emit("event_edit_presence", {})
                                .expect("failed to emit edit event");
                    }
                    "sp-clear" => {
                        app.get_window("main").unwrap()
                            .emit("event_clear_presence", {})
                                .expect("failed to emit clear event");
                    }
                    "sp-github" => {
                        open::that("https://github.com/dilanx/switchpresence").unwrap();
                    }
                    "sp-quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_discord_client() -> Client {
    let mut client = Client::new(995819278672601099);
    client.start();
    return client;
}

fn init_discord_client_win() -> DiscordIpcClient {
    let mut client = DiscordIpcClient::new("995819278672601099").unwrap();
    client.connect().unwrap();
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
                        AboutMetadata::new()
                            .copyright("Copyright © 2022 Dilan Nair (dilanxd.com)".to_string())
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
        let discord = Submenu::new(
            "Discord",
            Menu::new()
                .add_item(CustomMenuItem::new("sp-clear", "Clear Activity").accelerator("CmdOrCtrl+K"))
        );
        menu = menu
            .add_submenu(about)
            .add_submenu(discord);
    }
    
    return menu;
    
}

fn init_system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("sp-focus", "Focus Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("sp-edit", "Edit Activity"))
        .add_item(CustomMenuItem::new("sp-clear", "Clear Activity"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("sp-github", "View on GitHub"))
        .add_item(CustomMenuItem::new("sp-quit", "Quit"))
    )
}

#[tauri::command(async)]
fn update_presence(game: String, state: State<App>) -> bool {
    if cfg!(target_os = "windows") {
        let mut client_win = state.client_win.lock().unwrap();
        (*client_win).set_activity(
            activity::Activity::new()
                .state(&game)
                .assets(
                    activity::Assets::new()
                        .large_image("switch")
                        .large_text(&game)
                        .small_image("green")
                        .small_text("SwitchPresence")
                )
                .timestamps(
                    activity::Timestamps::new()
                        .start(SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap().as_secs().try_into().unwrap())
                )
        ).unwrap();
        return true;
    }

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
                .unwrap().as_secs();
            ts.start(since_the_epoch)
        });
    let mut client = state.client.lock().unwrap();
    let act = (*client).set_activity(|_| activity);
    return act.is_ok();
}

#[tauri::command(async)]
fn clear_presence(state: State<App>) -> bool {
    if cfg!(target_os = "windows") {
        let mut client_win = state.client_win.lock().unwrap();
        return (*client_win).set_activity(activity::Activity::new()).is_ok();
    }
    let mut client = state.client.lock().unwrap();
    return (*client).clear_activity().is_ok();
}

use std::error::Error;

fn test() -> Result<(), Box<dyn Error>> {
    let i = 0;
    if i == 1 {
        
    }
    Ok(())
}