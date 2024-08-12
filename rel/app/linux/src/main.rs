#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let open = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let new_notebook = tauri::CustomMenuItem::new("new_notebook".to_string(), "New Notebook");

    let copy_url = tauri::CustomMenuItem::new("copy_url".to_string(), "Copy URL").disabled();

    let view_logs = tauri::CustomMenuItem::new("view_logs".to_string(), "View Logs");
    let config_file =
        tauri::CustomMenuItem::new("config_file".to_string(), "Open .livebookdesktop.sh");
    let settings = tauri::CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(open)
        .add_item(new_notebook)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(copy_url)
        .add_item(view_logs)
        .add_item(config_file)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(quit);

    let system_tray = tauri::SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running Livebook");
}
