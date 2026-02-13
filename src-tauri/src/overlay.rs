use tauri::{AppHandle, Manager, PhysicalPosition};

/// Show the overlay window at top-center of the primary monitor.
/// Does NOT steal focus — the window was configured with `focus: false`.
pub fn show_overlay(app: &AppHandle) {
    let Some(window) = app.get_webview_window("overlay") else {
        eprintln!("[overlay] overlay window not found");
        return;
    };

    // Position at top-center of primary monitor
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let monitor_pos = monitor.position();
        let monitor_size = monitor.size();
        let scale = monitor.scale_factor();

        // Overlay physical size (360×80 logical → physical)
        let overlay_width = (360.0 * scale) as i32;

        // Center horizontally on the monitor
        let x = monitor_pos.x + (monitor_size.width as i32 - overlay_width) / 2;

        // Vertical offset: 40px logical from top (accounts for macOS menu bar ~24px + padding)
        let y = monitor_pos.y + (40.0 * scale) as i32;

        let _ = window.set_position(PhysicalPosition::new(x, y));
    }

    let _ = window.show();
    // Intentionally NOT calling set_focus() — the overlay must not steal focus
}

/// Hide the overlay window.
pub fn hide_overlay(app: &AppHandle) {
    let Some(window) = app.get_webview_window("overlay") else {
        return;
    };
    let _ = window.hide();
}
