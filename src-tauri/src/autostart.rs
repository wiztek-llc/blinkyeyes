use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

pub fn set_autostart(app: &AppHandle, enabled: bool) {
    let autolaunch = app.autolaunch();
    let result = if enabled {
        autolaunch.enable()
    } else {
        autolaunch.disable()
    };
    if let Err(e) = result {
        eprintln!("[autostart] Failed to set autostart: {}", e);
    }
}
