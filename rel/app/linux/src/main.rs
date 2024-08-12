#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_context;
use tauri::AppHandle;
use tauri::SystemTrayMenuItemHandle;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

enum TrayIconId {
    Open,
    NewNotebook,
    CopyURL,
    ViewLogs,
    BootScript,
    Settings,
    Quit,
}

impl TrayIconId {
    fn as_str(&self) -> &'static str {
        match self {
            TrayIconId::Open => "open",
            TrayIconId::NewNotebook => "new_notebook",
            TrayIconId::CopyURL => "copy_url",
            TrayIconId::ViewLogs => "view_logs",
            TrayIconId::BootScript => "boot_script",
            TrayIconId::Settings => "settings",
            TrayIconId::Quit => "quit",
        }
    }
}

fn build_system_tray() -> SystemTray {
    let open = CustomMenuItem::new(TrayIconId::Open.as_str(), "Open");
    let new_notebook = CustomMenuItem::new(TrayIconId::NewNotebook.as_str(), "New Notebook");

    let copy_url = CustomMenuItem::new(TrayIconId::CopyURL.as_str(), "Copy URL").disabled();

    let view_logs = CustomMenuItem::new(TrayIconId::ViewLogs.as_str(), "View Logs");
    let boot_script =
        CustomMenuItem::new(TrayIconId::BootScript.as_str(), "Open .livebookdesktop.sh");
    let settings = CustomMenuItem::new(TrayIconId::Settings.as_str(), "Settings");
    let quit = CustomMenuItem::new(TrayIconId::Quit.as_str(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(new_notebook)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(copy_url)
        .add_item(view_logs)
        .add_item(boot_script)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn open_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    // elixirkit::api::publish("open", "");
}

fn new_notebook_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    // elixirkit::api::publish("open", "/new");
}

fn copy_url_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    //
}

fn view_logs_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    // open file
}

fn open_boot_script_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    let path = std::path::Path::new("~/").join(".livebookdesktop.sh");

    if !path.exists() {
        let contents = "
# This file is used to configure Livebook before booting.
# If you change this file, you must restart Livebook for your changes to take place.
# See https://hexdocs.pm/livebook/readme.html#environment-variables for all available environment variables.

# # Allow Livebook to connect to remote machines over IPv6
# export ERL_AFLAGS=\"-proto_dist inet6_tcp\"
    ";

        std::fs::write(path, contents).expect("could not write the boot script file");
    }

    // open file
}

fn settings_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    // elixirkit::api::publish("open", "/settings");
}

fn quit_clicked(_app: AppHandle, _item_handle: SystemTrayMenuItemHandle) {
    std::process::exit(0);
}

fn main() {
    tauri::Builder::default()
        .system_tray(build_system_tray())
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                let item_handle = app.tray_handle().get_item(&id);

                match id.as_str() {
                    "open" => open_clicked(app.clone(), item_handle),
                    "new_notebook" => new_notebook_clicked(app.clone(), item_handle),
                    "copy_url" => copy_url_clicked(app.clone(), item_handle),
                    "view_logs" => view_logs_clicked(app.clone(), item_handle),
                    "boot_script" => open_boot_script_clicked(app.clone(), item_handle),
                    "settings" => settings_clicked(app.clone(), item_handle),
                    "quit" => quit_clicked(app.clone(), item_handle),
                    _ => {}
                }
            }
        })
        .run(generate_context!())
        .expect("error while running Livebook");
}
